#!/usr/bin/env node

/**
 * Skrypt generujący statystyki tłumaczeń
 * Tworzy raport o pokryciu tłumaczeń dla każdego języka
 */

const fs = require('fs');
const path = require('path');

// Ścieżka do plików tłumaczeń
const LOCALES_DIR = path.join(__dirname, '../src/i18n/locales');

// Lista obsługiwanych języków
const LANGUAGES = {
  'en': { name: 'English', nativeName: 'English' },
  'pl': { name: 'Polish', nativeName: 'Polski' },
  'de': { name: 'German', nativeName: 'Deutsch' },
  'fr': { name: 'French', nativeName: 'Français' },
  'es': { name: 'Spanish', nativeName: 'Español' },
  'it': { name: 'Italian', nativeName: 'Italiano' },
  'ru': { name: 'Russian', nativeName: 'Русский' },
  'uk': { name: 'Ukrainian', nativeName: 'Українська' },
  'zh-CN': { name: 'Chinese (Simplified)', nativeName: '简体中文' },
  'ja': { name: 'Japanese', nativeName: '日本語' },
  'ko': { name: 'Korean', nativeName: '한국어' }
};

/**
 * Wczytaj plik JSON
 */
function loadJSON(filePath) {
  try {
    const content = fs.readFileSync(filePath, 'utf8');
    return JSON.parse(content);
  } catch (error) {
    return null;
  }
}

/**
 * Policz klucze w obiekcie tłumaczeń
 */
function countKeys(obj) {
  let count = 0;
  
  function traverse(o) {
    for (const [key, value] of Object.entries(o)) {
      if (typeof value === 'object' && value !== null) {
        traverse(value);
      } else {
        count++;
      }
    }
  }
  
  traverse(obj);
  return count;
}

/**
 * Policz przetłumaczone klucze (niepuste wartości)
 */
function countTranslatedKeys(obj) {
  let count = 0;
  let total = 0;
  
  function traverse(o) {
    for (const [key, value] of Object.entries(o)) {
      if (typeof value === 'object' && value !== null) {
        traverse(value);
      } else {
        total++;
        if (typeof value === 'string' && value.trim() !== '') {
          count++;
        }
      }
    }
  }
  
  traverse(obj);
  return { translated: count, total };
}

/**
 * Oblicz pokrycie tłumaczeń w procentach
 */
function calculateCoverage(translated, total) {
  if (total === 0) return 0;
  return Math.round((translated / total) * 100);
}

/**
 * Pobierz statystyki dla języka
 */
function getLanguageStats(lang) {
  const filePath = path.join(LOCALES_DIR, `${lang}.json`);
  const translations = loadJSON(filePath);
  
  if (!translations) {
    return null;
  }
  
  const totalKeys = countKeys(translations);
  const { translated, total } = countTranslatedKeys(translations);
  const coverage = calculateCoverage(translated, total);
  const fileInfo = fs.statSync(filePath);
  const fileSizeKB = Math.round(fileInfo.size / 1024);
  
  return {
    lang,
    langName: LANGUAGES[lang]?.name || lang,
    nativeName: LANGUAGES[lang]?.nativeName || lang,
    totalKeys,
    translatedKeys: translated,
    untranslatedKeys: total - translated,
    coverage,
    fileSizeKB
  };
}

/**
 * Generuj pasek postępu
 */
function generateProgressBar(percentage, width = 30) {
  const filled = Math.round((percentage / 100) * width);
  const empty = width - filled;
  
  let bar = '';
  
  if (percentage >= 90) {
    bar = '█'.repeat(filled);
  } else if (percentage >= 70) {
    bar = '▓'.repeat(filled);
  } else if (percentage >= 50) {
    bar = '▒'.repeat(filled);
  } else {
    bar = '░'.repeat(filled);
  }
  
  bar += '░'.repeat(empty);
  
  return bar;
}

/**
 * Main
 */
function main() {
  const stats = [];
  
  console.log('📊 Generating translation statistics...\n');
  
  // Pobierz statystyki dla każdego języka
  for (const lang of Object.keys(LANGUAGES)) {
    const stat = getLanguageStats(lang);
    if (stat) {
      stats.push(stat);
    }
  }
  
  // Sortuj według pokrycia (malejąco)
  stats.sort((a, b) => b.coverage - a.coverage);
  
  // Generuj raport Markdown
  let markdown = '# Translation Statistics 🌍\n\n';
  markdown += `Generated: ${new Date().toISOString()}\n\n`;
  
  // Podsumowanie ogólne
  const avgCoverage = Math.round(stats.reduce((sum, s) => sum + s.coverage, 0) / stats.length);
  markdown += '## Summary\n\n';
  markdown += `- **Languages**: ${stats.length}\n`;
  markdown += `- **Average Coverage**: ${avgCoverage}%\n`;
  markdown += `- **Total Keys**: ${stats[0]?.totalKeys || 0}\n\n`;
  
  // Tabela statystyk
  markdown += '## Language Coverage\n\n';
  markdown += '| Language | Native Name | Coverage | Translated | Untranslated | Size |\n';
  markdown += '|----------|-------------|----------|------------|--------------|------|\n';
  
  for (const stat of stats) {
    const coverageBar = generateProgressBar(stat.coverage);
    markdown += `| ${stat.langName} | ${stat.nativeName} | ${coverageBar} ${stat.coverage}% | ${stat.translatedKeys}/${stat.totalKeys} | ${stat.untranslatedKeys} | ${stat.fileSizeKB}KB |\n`;
  }
  
  // Szczegółowe statystyki
  markdown += '\n## Detailed Statistics\n\n';
  
  for (const stat of stats) {
    const coverageBar = generateProgressBar(stat.coverage, 40);
    markdown += `### ${stat.nativeName} (${stat.lang})\n\n`;
    markdown += `Coverage: ${coverageBar} ${stat.coverage}%\n\n`;
    markdown += `- **Translated**: ${stat.translatedKeys} keys\n`;
    markdown += `- **Untranslated**: ${stat.untranslatedKeys} keys\n`;
    markdown += `- **Total Keys**: ${stat.totalKeys}\n`;
    markdown += `- **File Size**: ${stat.fileSizeKB}KB\n\n`;
  }
  
  // Status języków
  markdown += '## Language Status\n\n';
  
  const completeLanguages = stats.filter(s => s.coverage === 100);
  const nearlyCompleteLanguages = stats.filter(s => s.coverage >= 90 && s.coverage < 100);
  const inProgressLanguages = stats.filter(s => s.coverage >= 50 && s.coverage < 90);
  const needsWorkLanguages = stats.filter(s => s.coverage < 50);
  
  if (completeLanguages.length > 0) {
    markdown += '### ✅ Complete (100%)\n';
    completeLanguages.forEach(l => {
      markdown += `- ${l.langName} (${l.lang})\n`;
    });
    markdown += '\n';
  }
  
  if (nearlyCompleteLanguages.length > 0) {
    markdown += '### 🟡 Nearly Complete (90-99%)\n';
    nearlyCompleteLanguages.forEach(l => {
      markdown += `- ${l.langName} (${l.lang}): ${l.coverage}%\n`;
    });
    markdown += '\n';
  }
  
  if (inProgressLanguages.length > 0) {
    markdown += '### 🟠 In Progress (50-89%)\n';
    inProgressLanguages.forEach(l => {
      markdown += `- ${l.langName} (${l.lang}): ${l.coverage}%\n`;
    });
    markdown += '\n';
  }
  
  if (needsWorkLanguages.length > 0) {
    markdown += '### 🔴 Needs Work (< 50%)\n';
    needsWorkLanguages.forEach(l => {
      markdown += `- ${l.langName} (${l.lang}): ${l.coverage}%\n`;
    });
    markdown += '\n';
  }
  
  // Zapisz raport
  fs.writeFileSync(path.join(__dirname, '../TRANSLATION_STATS.md'), markdown);
  
  // Zapisz JSON dla automatycznego przetwarzania
  const jsonStats = JSON.stringify(stats, null, 2);
  fs.writeFileSync(path.join(__dirname, '../translation-coverage-report.json'), jsonStats);
  
  // Wyświetl w konsoli
  console.log('📈 Translation Statistics\n');
  console.log('━'.repeat(80));
  
  for (const stat of stats) {
    const coverageBar = generateProgressBar(stat.coverage);
    console.log(`${stat.langName.padEnd(20)} | ${coverageBar} ${stat.coverage.toString().padStart(3)}% | ${stat.translatedKeys}/${stat.totalKeys} keys`);
  }
  
  console.log('━'.repeat(80));
  console.log(`\n✅ Translation statistics saved to TRANSLATION_STATS.md`);
  console.log(`📊 JSON report saved to translation-coverage-report.json`);
}

if (require.main === module) {
  main();
}

module.exports = { getLanguageStats, calculateCoverage };