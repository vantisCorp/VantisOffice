#!/usr/bin/env node

/**
 * Skrypt walidujący pliki tłumaczeń
 * Sprawdza:
 * - Poprawność składni JSON
 * - Obecność wszystkich wymaganych kluczy
 * - Spójność struktur tłumaczeń
 */

const fs = require('fs');
const path = require('path');

// Ścieżka do plików tłumaczeń
const LOCALES_DIR = path.join(__dirname, '../src/i18n/locales');

// Wymagane klucze w tłumaczeniach
const REQUIRED_KEYS = [
  'common.save',
  'common.cancel',
  'common.delete',
  'common.edit',
  'editor.newDocument',
  'editor.openDocument',
  'editor.saveDocument',
  'editor.exportPDF',
  'format.bold',
  'format.italic',
  'format.underline',
  'format.strikethrough'
];

/**
 * Wczytaj plik JSON
 */
function loadJSON(filePath) {
  try {
    const content = fs.readFileSync(filePath, 'utf8');
    return JSON.parse(content);
  } catch (error) {
    console.error(`❌ Error loading ${filePath}:`, error.message);
    return null;
  }
}

/**
 * Sprawdź czy klucz istnieje w obiekcie (obsługuje zagnieżdżone klucze)
 */
function hasKey(obj, key) {
  const keys = key.split('.');
  let current = obj;
  
  for (const k of keys) {
    if (!current || !current.hasOwnProperty(k)) {
      return false;
    }
    current = current[k];
  }
  
  return true;
}

/**
 * Waliduj pojedynczy plik tłumaczeń
 */
function validateTranslationFile(filePath, lang) {
  const translations = loadJSON(filePath);
  
  if (!translations) {
    return {
      valid: false,
      errors: [`Failed to load ${filePath}`]
    };
  }
  
  const errors = [];
  const warnings = [];
  
  // Sprawdź wymagane klucze
  for (const key of REQUIRED_KEYS) {
    if (!hasKey(translations, key)) {
      errors.push(`Missing required key: ${key}`);
    }
  }
  
  // Sprawdź czy wartości nie są puste
  function checkEmptyValues(obj, prefix = '') {
    for (const [key, value] of Object.entries(obj)) {
      const fullKey = prefix ? `${prefix}.${key}` : key;
      
      if (typeof value === 'object' && value !== null) {
        checkEmptyValues(value, fullKey);
      } else if (typeof value === 'string') {
        if (value.trim() === '') {
          warnings.push(`Empty value for key: ${fullKey}`);
        }
        
        // Sprawdź czy nie ma interpolacji bez odpowiednich zmiennych
        if (value.includes('{') && !value.includes('}')) {
          warnings.push(`Possible incomplete interpolation in: ${fullKey}`);
        }
      }
    }
  }
  
  checkEmptyValues(translations);
  
  // Sprawdź czy nie ma duplikatów
  function checkDuplicates(obj, prefix = '', seen = new Map()) {
    for (const [key, value] of Object.entries(obj)) {
      const fullKey = prefix ? `${prefix}.${key}` : key;
      
      if (typeof value === 'object' && value !== null) {
        checkDuplicates(value, fullKey, seen);
      } else if (typeof value === 'string') {
        if (seen.has(value)) {
          warnings.push(`Duplicate translation value: "${value}" in keys: ${seen.get(value)} and ${fullKey}`);
        }
        seen.set(value, fullKey);
      }
    }
  }
  
  checkDuplicates(translations);
  
  return {
    valid: errors.length === 0,
    errors,
    warnings
  };
}

/**
 * Porównaj strukturę tłumaczeń z bazowym językiem (angielski)
 */
function compareStructure(baseLang, targetLang) {
  const basePath = path.join(LOCALES_DIR, `${baseLang}.json`);
  const targetPath = path.join(LOCALES_DIR, `${targetLang}.json`);
  
  const baseTranslations = loadJSON(basePath);
  const targetTranslations = loadJSON(targetPath);
  
  if (!baseTranslations || !targetTranslations) {
    return;
  }
  
  const missingKeys = [];
  
  function findMissing(baseObj, targetObj, prefix = '') {
    for (const [key, value] of Object.entries(baseObj)) {
      const fullKey = prefix ? `${prefix}.${key}` : key;
      
      if (!targetObj.hasOwnProperty(key)) {
        missingKeys.push(fullKey);
        continue;
      }
      
      if (typeof value === 'object' && typeof targetObj[key] === 'object') {
        findMissing(value, targetObj[key], fullKey);
      }
    }
  }
  
  findMissing(baseTranslations, targetTranslations);
  
  if (missingKeys.length > 0) {
    console.warn(`⚠️  ${targetLang} is missing ${missingKeys.length} keys compared to ${baseLang}`);
    missingKeys.forEach(key => console.warn(`   - ${key}`));
  }
}

/**
 * Main
 */
function main() {
  console.log('🔍 Validating translations...\n');
  
  // Pobierz wszystkie pliki tłumaczeń
  const translationFiles = fs.readdirSync(LOCALES_DIR)
    .filter(file => file.endsWith('.json'))
    .sort();
  
  if (translationFiles.length === 0) {
    console.error('❌ No translation files found in', LOCALES_DIR);
    process.exit(1);
  }
  
  console.log(`📁 Found ${translationFiles.length} translation files\n`);
  
  let allValid = true;
  const results = [];
  
  // Waliduj każdy plik
  for (const file of translationFiles) {
    const lang = path.basename(file, '.json');
    const filePath = path.join(LOCALES_DIR, file);
    
    console.log(`\n📝 Validating ${lang}...`);
    const result = validateTranslationFile(filePath, lang);
    
    results.push({ lang, ...result });
    
    if (result.errors.length > 0) {
      console.error('❌ Errors:');
      result.errors.forEach(err => console.error(`   - ${err}`));
      allValid = false;
    }
    
    if (result.warnings.length > 0) {
      console.warn('⚠️  Warnings:');
      result.warnings.forEach(warn => console.warn(`   - ${warn}`));
    }
    
    if (result.valid) {
      console.log('✅ Valid');
    }
  }
  
  // Porównaj strukturę z angielskim
  console.log('\n\n🔗 Comparing structure with English...');
  const englishFile = translationFiles.find(f => f.startsWith('en'));
  if (englishFile) {
    for (const file of translationFiles) {
      if (file !== englishFile) {
        const lang = path.basename(file, '.json');
        compareStructure('en', lang);
      }
    }
  }
  
  // Podsumowanie
  console.log('\n\n📊 Validation Summary');
  console.log('━'.repeat(50));
  console.log(`Total files: ${results.length}`);
  console.log(`Valid: ${results.filter(r => r.valid).length}`);
  console.log(`Invalid: ${results.filter(r => !r.valid).length}`);
  console.log(`Total errors: ${results.reduce((sum, r) => sum + r.errors.length, 0)}`);
  console.log(`Total warnings: ${results.reduce((sum, r) => sum + r.warnings.length, 0)}`);
  console.log('━'.repeat(50));
  
  if (!allValid) {
    console.error('\n❌ Translation validation failed');
    process.exit(1);
  }
  
  console.log('\n✅ All translations are valid');
}

if (require.main === module) {
  main();
}

module.exports = { validateTranslationFile, compareStructure };