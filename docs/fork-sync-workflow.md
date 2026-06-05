# Fork sync workflow

Цей документ описує безпечний порядок синхронізації fork `origin/main` з `upstream/main`.
Мета: не втратити локальні коміти, не запустити повторний rebase випадково і не пушити стан,
який ще не перевірений.

## Перед будь-якими змінами

Завжди починай з перевірки стану:

```powershell
git status
git branch -vv
git rev-parse --abbrev-ref HEAD
git log --oneline --decorate --graph --all -20
```

Якщо Git показує активний `rebase`, `merge`, `cherry-pick` або `bisect`, не запускай новий rebase.
Спочатку треба завершити або скасувати активну операцію за явною згодою користувача.

## Що означають origin, upstream і local

- `local main` - твоя локальна гілка `main`.
- `origin/main` - твій fork на GitHub.
- `upstream/main` - основний репозиторій Claw Code, з якого fork бере оновлення.

Зазвичай локальні зміни мають жити у `main`, потім пушитись в `origin/main`.
Оновлення з основного проєкту приходять з `upstream/main`.

## Коли rebase дозволений

Rebase на `upstream/main` можна робити тільки коли:

- ти на гілці `main`;
- `git status` показує clean working tree;
- немає активного rebase/merge/cherry-pick/bisect;
- ти вже зробив `git fetch upstream` і `git fetch origin`;
- ти перевірив різницю через `--cherry-pick`;
- є явна згода користувача на rebase.

## Безпечний rebase

Перевірити оновлення:

```powershell
scripts/check-upstream-updates.ps1
```

Якщо upstream має унікальні коміти і користувач дозволив rebase:

```powershell
git rebase upstream/main
```

Після rebase перевір:

```powershell
git status
git log --oneline --decorate --graph --all -20
```

Не запускай `git push --force-with-lease origin main`, доки не перевірено, що rebase завершений,
working tree clean, і користувач явно дозволив push.

## Коли НЕ робити rebase

Не запускай rebase, якщо:

- rebase вже активний;
- є conflicts;
- working tree dirty;
- є untracked файли, які можуть бути перезаписані;
- поточна гілка не `main`;
- користувач просив тільки перевірити стан;
- немає зрозумілого плану відновлення після конфлікту.

## Як перевірити оновлення через --cherry-pick

Оновити refs:

```powershell
git fetch upstream
git fetch origin
```

Порівняти `upstream/main` і `main`:

```powershell
git log --oneline --decorate --graph --left-right --cherry-pick upstream/main...main
git rev-list --count --cherry-pick --left-only upstream/main...main
git rev-list --count --cherry-pick --right-only upstream/main...main
```

У цьому порівнянні:

- `<` означає коміт тільки в `upstream/main`;
- `>` означає коміт тільки в локальному `main`;
- `--cherry-pick` прибирає patch-equivalent коміти, тобто коміти з різними SHA, але однаковими змінами.

Без `--cherry-pick` після rebase можна побачити оманливі дублікати: Git покаже різні SHA з обох боків,
навіть якщо частина змін уже фактично застосована.

## Як перевірити локальні коміти

Показати коміти, які є тільки в локальному `main`:

```powershell
git log --oneline --decorate --graph --right-only --cherry-pick upstream/main...main
```

Порахувати їх:

```powershell
git rev-list --count --cherry-pick --right-only upstream/main...main
```

## Як безпечно push після rebase

Push після rebase дозволений тільки якщо:

- rebase завершений;
- `git status` clean;
- поточна гілка `main`;
- `origin/main` перевірений через `git fetch origin`;
- користувач явно дозволив push.

Команда:

```powershell
git push --force-with-lease origin main
```

`--force-with-lease` безпечніший за `--force`, бо не перезапише чужі нові зміни в `origin/main`,
якщо вони з'явилися після останнього fetch.

## Заборонені команди без явного дозволу

Не виконувати автоматично:

```powershell
git reset --hard
git clean -fdx
git rebase --skip
git rebase --abort
git push --force
git push --force-with-lease origin main
```

`git rebase --abort` дозволений тільки коли користувач прямо попросив скасувати активний rebase
або дав явну згоду на recovery.

## Якщо випадково стартував повторний rebase

1. Зупинись і не запускай `git rebase --continue` або `git rebase --skip`.
2. Перевір стан:

```powershell
git status
git branch -vv
git rev-parse --abbrev-ref HEAD
git log --oneline --decorate --graph --all -20
```

3. Якщо користувач дозволив abort, виконай:

```powershell
git rebase --abort
```

4. Якщо abort блокується untracked файлами, спочатку зроби backup цих файлів поза working tree.
5. Після abort знову перевір:

```powershell
git status
git branch -vv
git log --oneline --decorate --graph -10
```

6. Не пушити після recovery без окремої явної згоди користувача.
