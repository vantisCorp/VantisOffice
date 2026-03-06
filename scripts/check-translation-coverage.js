#!/usr/bin/env node

/**
 * Skrypt sprawdzający pokrycie tłumaczeń
 * Wymusza minimalne pokrycie tłumaczeń dla wszystkich języków
 */

const fs = require('fs');
const path = require('path');

// Minimalne wymagane pokrycie dla każdego języka
const MINIMUM_COVERAGE = {
  'en': 100,  // Angielski - bazowy język
  'pl': 95,   // Polski - język główny
  'de': 80,   // Niemiecki - ważny rynek
  'fr': 80,   // Francuski - ważny rynek
  'es': 80,   // Hiszpański - ważny rynek
  'it': 70,   // Włoski - standard
  'ru': 70,   // Rosyjski - standard
  'uk': 70,   // Ukraiński - standard
  'zh-CN': 70, // Chiński - ważny rynek
  'ja': 70,   // Japoński - ważny rynek
  'ko': 70    // Koreański - standard
};

/**
 * Wczytaj raport pokrycia tłumaczeń
 */
function loadCoverageReport() {
  const reportPath = path.join(__dirname, '../translation-coverage-report.json');
  
  if (!fs.existsSync(reportPath)) {
    console.error('❌ Translation coverage report not found');
    console.error('Run: npm run generate-translation-stats');
    process.exit(1);
  }
  
  const content = fs.readFileSync(reportPath, 'utf8');
  return JSON.parse(content);
}

/**
 * Sprawdź czy pokrycie spełnia wymagania
 */
function checkCoverage(stats) {
  const issues = [];
  const warnings = [];
  const passed = [];
  
  for (const stat of stats) {
    const minCoverage = MINIMUM_COVERAGE[stat.lang];
    
    if (minCoverage === undefined) {
      warnings.push({
        lang: stat.lang,
        message: `No minimum coverage defined for ${stat.lang}`
      });
      continue;
    }
    
    if (stat.coverage < minCoverage) {
      issues.push({
        lang: stat.lang,
        current: stat.coverage,
        required: minCoverage,
        diff: minCoverage - stat.coverage,
        message: `${stat.langName}: ${stat.coverage}% (required: ${minCoverage}%, missing: ${minCoverage - stat.coverage}%)`
      });
    } else {
      passed.push({
        lang: stat.lang,
        coverage: stat.coverage,
        message: `${stat.langName}: ${stat.coverage}% ✅`
      });
    }
  }
  
  return { issues, warnings, passed };
}

/**
 * Generuj pasek postępu
 */
function generateProgressBar(percentage, required, width = 30) {
  const filled = Math.round((percentage / 100) * width);
  const empty = width - filled;
  
  let bar = '';
  
  if (percentage >= required) {
    bar = '█'.repeat(filled);
  } else if (percentage >= required * 0.9) {
    bar = '▓'.repeat(filled);
  } else if (percentage >= required * 0.7) {
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
  console.log('🔍 Checking translation coverage...\n');
  
  // Wczytaj statystyki
  const stats = loadCoverageReport();
  
  if (!stats || stats.length === 0) {
    console.error('❌ No translation statistics found');
    process.exit(1);
  }
  
  // Sprawdź pokrycie
  const { issues, warnings, passed } = checkCoverage(stats);
  
  // Wyświetl wyniki
  console.log('📊 Coverage Check Results\n');
  console.log('━'.repeat(80));
  
  if (passed.length > 0) {
    console.log('\n✅ Passed Languages:\n');
    for (const lang of passed) {
      const required = MINIMUM_COVERAGE[lang.lang];
      const bar = generateProgressBar(lang.coverage, required);
      console.log(`  ${lang.message} ${bar}`);
    }
  }
  
  if (issues.length > 0) {
    console.log('\n❌ Languages Below Minimum Coverage:\n');
    for (const issue of issues) {
      const bar = generateProgressBar(issue.current, issue.required);
      console.log(`  ${issue.message} ${bar}`);
      console.log(`    Missing: ${issue.diff}% (${issue.current}/${issue.required} keys)`);
    }
  }
  
  if (warnings.length > 0) {
    console.log('\n⚠️  Warnings:\n');
    for (const warning of warnings) {
      console.log(`  ${warning.message}`);
    }
  }
  
  console.log('\n' + '━'.repeat(80));
  
  // Podsumowanie
  console.log(`\n📈 Summary:`);
  console.log(`  Passed: ${passed.length}`);
  console.log(`  Failed: ${issues.length}`);
  console.log(`  Warnings: ${warnings.length}`);
  
  // Zakończ z odpowiednim kodem
  if (issues.length > 0) {
    console.error('\n❌ Translation coverage check failed');
    console.error('\nTo fix:');
    console.error('1. Update translations in Weblate');
    console.error('2. Or adjust MINIMUM_COVERAGE in check-translation-coverage.js');
    console.error('3. Run: npm run generate-translation-stats');
    process.exit(1);
  }
  
  if (warnings.length > 0) {
    console.warn('\n⚠️  Translation coverage check passed with warnings');
  }
  
  console.log('\n✅ All languages meet minimum coverage requirements');
}

if (require.main === module) {
  main();
}

module.exports = { checkCoverage };