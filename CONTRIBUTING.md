# Przewodnik dla współtwórców VantisOffice

Dziękujemy za zainteresowanie współtworzeniem VantisOffice! Ten przewodnik pomoże Ci w procesie wnoszenia wkładu do projektu.

## Spis treści

- [Wymagania wstępne](#wymagania-wstępne)
- [Konfiguracja środowiska deweloperskiego](#konfiguracja-środowiska-deweloperskiego)
- [Proces rozwoju](#proces-rozwoju)
- [Praca z Git](#praca-z-git)
- [Standardy kodowania](#standardy-kodowania)
- [Testowanie](#testowanie)
- [Dokumentacja](#dokumentacja)
- [Proces pull request](#proces-pull-request)
- [Kodeks postępowania](#kodeks-postępowania)

## Wymagania wstępne

Przed rozpoczęciem pracy nad projektem upewnij się, że masz zainstalowane:

- **Node.js** (wersja 18.x lub nowsza)
- **npm** lub **yarn** (zalecany npm)
- **Git** (wersja 2.x lub nowsza)
- **VS Code** lub preferowane IDE (zalecany VS Code)

## Konfiguracja środowiska deweloperskiego

### 1. Forkuj repozytorium

1. Otwórz [VantisOffice repository](https://github.com/vantisCorp/VantisOffice)
2. Kliknij przycisk "Fork" w prawym górnym rogu
3. Skopiuj URL swojego forka

### 2. Klonuj repozytorium

```bash
# Klonuj swój fork
git clone https://github.com/TWOJ_USERNAME/VantisOffice.git
cd VantisOffice

# Dodaj upstream repozytorium
git remote add upstream https://github.com/vantisCorp/VantisOffice.git

# Weryfikuj remote
git remote -v
```

### 3. Instalacja zależności

```bash
# Instaluj zależności
npm install

# Instaluj zależności deweloperskie
npm install --save-dev

# Skompiluj projekt (jeśli wymagane)
npm run build
```

### 4. Konfiguracja pre-commit hooks (opcjonalnie)

```bash
# Instaluj Husky
npm install husky --save-dev

# Konfiguruj pre-commit hooks
npx husky install
npx husky add .husky/pre-commit "npm run lint && npm run test"
```

## Proces rozwoju

### Wybór zadania

1. Sprawdź [Issues](https://github.com/vantisCorp/VantisOffice/issues) dla otwartych zadań
2. Poszukaj oznaczeń:
   - `good first issue` - dla początkujących współtwórców
   - `help wanted` - zadania wymagające pomocy
   - `enhancement` - nowe funkcjonalności
   - `bug` - naprawy błędów

3. Skomentuj issue oznaczone jako `up-for-grabs` aby zarezerwować zadanie

### Tworzenie brancha

```bash
# Upewnij się, że Twój branch jest aktualny
git checkout main
git pull upstream main

# Utwórz nowy branch dla swojego zadania
git checkout -b feature/opis-zadania
# lub
git checkout -b fix/opis-bledu
# lub
git checkout -b docs/opis-dokumentacji
```

**Konwencje nazewnictwa branchów:**
- `feature/` - nowe funkcjonalności
- `fix/` - naprawy błędów
- `docs/` - aktualizacje dokumentacji
- `refactor/` - refaktoryzacja
- `test/` - testy
- `chore/` - konfiguracja i narzędzia

## Praca z Git

### Commit messages

Używaj konwencji [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Typy commitów:**
- `feat`: nowa funkcjonalność
- `fix`: naprawa błędu
- `docs`: zmiany w dokumentacji
- `style`: zmiany formatowania (nie zmieniają kodu)
- `refactor`: refaktoryzacja
- `perf`: ulepszenia wydajności
- `test`: testy
- `chore`: konfiguracja i narzędzia
- `ci`: zmiany w CI/CD
- `revert`: cofnięcie commita

**Przykłady:**
```bash
git commit -m "feat(editor): dodaj obsługę formatowania list numerowanych"
git commit -m "fix(api): napraw błąd przy zapisywaniu dokumentu"
git commit -m "docs(readme): aktualizuj instrukcje instalacji"
```

### Częste commity

```bash
# Dodaj zmienione pliki
git add .

# Zatwierdź zmiany
git commit -m "type(scope): opis zmian"
```

## Standardy kodowania

### JavaScript/TypeScript

1. **Formatowanie kodu:**
   - Używaj Prettier dla automatycznego formatowania
   - Uruchom `npm run format` przed commitem

2. **Linting:**
   - Używaj ESLint dla sprawdzania jakości kodu
   - Uruchom `npm run lint` przed commitem

3. **Konwencje:**
   - Używaj `camelCase` dla zmiennych i funkcji
   - Używaj `PascalCase` dla klas i komponentów
   - Używaj `UPPER_SNAKE_CASE` dla stałych

```typescript
// Przykład
const userName = "John";
class DocumentEditor { }
const MAX_FILE_SIZE = 10485760;
```

4. **Komentarze:**
   - Dodawaj komentarze JSDoc dla funkcji publicznych
   - Komentuj skomplikowaną logikę

```typescript
/**
 * Otwiera dokument z określonej ścieżki
 * @param {string} path - Ścieżka do pliku dokumentu
 * @returns {Promise<Document>} Obiekt dokumentu
 */
async function openDocument(path: string): Promise<Document> {
  // implementacja
}
```

### CSS/SCSS

1. Używaj BEM (Block Element Modifier) dla nazw klas
2. Używaj zmiennych CSS dla wartości powtarzalnych
3. Organizuj style według komponentów

```scss
.button {
  &--primary {
    background: $primary-color;
  }
  
  &__icon {
    margin-right: 8px;
  }
}
```

## Testowanie

### Uruchamianie testów

```bash
# Wszystkie testy
npm test

# Testy w trybie watch
npm run test:watch

# Testowanie pojedynczego pliku
npm test -- src/components/Button.test.tsx

# Pokrycie kodu
npm run test:coverage
```

### Pisanie testów

1. **Testy jednostkowe:**
   - Testuj pojedyncze funkcje i metody
   - Używaj mocków dla zależności zewnętrznych

2. **Testy integracyjne:**
   - Testuj interakcje między komponentami
   - Symuluj działania użytkownika

3. **Testy E2E:**
   - Testuj pełne przepływy użytkownika
   - Używaj Playwright lub Cypress

**Przykład testu (Jest + React Testing Library):**
```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { Button } from './Button';

describe('Button', () => {
  it('should render button with text', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });

  it('should call onClick when clicked', () => {
    const handleClick = jest.fn();
    render(<Button onClick={handleClick}>Click me</Button>);
    fireEvent.click(screen.getByText('Click me'));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });
});
```

## Dokumentacja

### Aktualizacja dokumentacji

1. **README.md:**
   - Aktualizuj przy dodawaniu nowych funkcji
   - Utrzymuj przykłady użycia aktualne

2. **Komentarze w kodzie:**
   - Dodawaj JSDoc dla funkcji publicznych
   - Komentuj skomplikowaną logikę

3. **Dokumentacja API:**
   - Utrzymuj dokumentację API aktualną
   - Dodawaj przykłady użycia

### Struktura dokumentacji

```
docs/
├── guides/
│   ├── getting-started.md
│   ├── installation.md
│   └── configuration.md
├── api/
│   ├── reference.md
│   └── examples.md
├── components/
│   └── component-reference.md
└── contributing/
    ├── workflow.md
    └── code-of-conduct.md
```

## Proces pull request

### 1. Przygotuj brancha

```bash
# Upewnij się, że Twój branch jest aktualny
git checkout main
git pull upstream main

# Przełącz się na swój branch
git checkout feature/twoje-zadanie

# Scal najnowsze zmiany z main
git merge main
```

### 2. Rozwiąż konflikty (jeśli wystąpią)

```bash
# Rozwiąż konflikty w plikach
# Zatwierdź rozwiązane konflikty
git add .
git commit -m "fix: rozwiąż konflikty merge"
```

### 3. Uruchom testy

```bash
# Uruchom testy
npm test

# Sprawdź linting
npm run lint

# Sformatuj kod
npm run format
```

### 4. Wypchnij zmiany

```bash
# Wypchnij branch do swojego forka
git push origin feature/twoje-zadanie
```

### 5. Utwórz Pull Request

1. Otwórz swoją stronę forka na GitHub
2. Kliknij "Pull Request"
3. Wypełnij template PR
4. Dodaj odpowiednie oznaczenia:
   - `WIP` - praca w toku (Work in Progress)
   - `RFC` - prośba o komentarze (Request for Comments)
   - `breaking` - zmiana łamiąca zgodność
5. Przypisz recenzentów
6. Czekaj na recenzję

### 6. Zareaguj na recenzję

- Odpowiadaj na komentarze
- Wprowadzaj wymagane zmiany
- Oznacz komentarze jako rozwiązane

### 7. Scalanie PR

Po zaakceptowaniu PR:
1. Właściciel repozytorium scali PR
2. Twoja zmiana zostanie dołączona do main

### 8. Posprzątaj po sobie

```bash
# Usuń zmergowany branch lokalnie
git branch -d feature/twoje-zadanie

# Usuń branch na zdalnym repozytorium
git push origin --delete feature/twoje-zadanie

# Zaktualizuj lokalne main
git checkout main
git pull upstream main
```

## Code Review

### Jak przejrzeć pull request:

1. **Sprawdź cel i zakres:**
   - Czy PR realizuje zamierzony cel?
   - Czy zakres zmian jest odpowiedni?

2. **Sprawdź kod:**
   - Czy kod jest czytelny i zrozumiały?
   - Czy są problemy z wydajnością?
   - Czy są problemy z bezpieczeństwem?

3. **Sprawdź testy:**
   - Czy testy pokrywają nowy kod?
   - Czy testy przechodzą?

4. **Sprawdź dokumentację:**
   - Czy dokumentacja została zaktualizowana?
   - Czy komentarze są odpowiednie?

### Jak otrzymać review:

1. **Napisz dobry opis PR:**
   - Wyjaśnij, co i dlaczego zmieniasz
   - Dodaj zrzuty ekranu dla zmian wizualnych

2. **Oznacz odpowiednich recenzentów:**
   - Zna się na kodzie
   - Odpowiedzialny za obszar zmian

3. **Bądź otwarty na feedback:**
   - Dyskutuj o różnych podejściach
   - Ucz się z komentarzy

## Zasady jakości

### Wymagania przed mergem

- [ ] Wszystkie testy przechodzą
- [ ] Kod jest sformatowany (Prettier)
- [ ] Brak błędów linting (ESLint)
- [ ] Dokumentacja została zaktualizowana
- [ ] Changelog został zaktualizowany
- [ ] Code review zostało ukończone

### Współpraca

- **Bądź uprzejmy:** Szanuj innych współtwórców
- **Bądź konstruktywny:** Dostarczaj pomocne komentarze
- **Bądź cierpliwy:** Czekaj na recenzję i odpowiedzi
- **Pytaj o pomoc:** Jeśli utkniesz, zapytaj społeczność

## Zasoby i pomoc

### Dokumentacja

- [Dokumentacja projektu](../README.md)
- [API Reference](../docs/api/reference.md)
- [Komponenty](../docs/components/component-reference.md)

### Narzędzia

- [GitHub Documentation](https://docs.github.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [ESLint](https://eslint.org/)
- [Prettier](https://prettier.io/)
- [Jest](https://jestjs.io/)

### Kontakt

- **GitHub Issues:** Zgłaszanie błędów i propozycje
- **Discussions:** Dyskusje o projekcie
- **Discord/Slack:** Komunikacja w czasie rzeczywistym

## Pytania częste

### Gdzie mogę znaleźć zadania do wykonania?

Sprawdź [Issues](https://github.com/vantisCorp/VantisOffice/issues) i szukaj oznaczeń:
- `good first issue` - dla początkujących
- `help wanted` - wymagające pomocy
- `enhancement` - nowe funkcje

### Co jeśli moje zmiany są duże?

Podziel duże zmiany na mniejsze, niezależne pull requesty. To ułatwia review i testowanie.

### Co jeśli mam konflity podczas merge?

```bash
git pull upstream main
# Rozwiąż konflikty ręcznie
git add .
git commit -m "fix: rozwiąż konflikty merge"
git push origin feature/twoje-zadanie
```

### Jak mogę zgłosić błąd?

Użyj [GitHub Issues](https://github.com/vantisCorp/VantisOffice/issues) i użyj template dla błędów.

---

Dziękujemy za Twój wkład w rozwój VantisOffice! 🚀

Jeśli masz jakieś pytania, skontaktuj się z nami przez GitHub Issues lub Discussions.