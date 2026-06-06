# Claw local workflow

Цей документ описує роботу з локальною гілкою `claw-local`.

## Що таке `claw-local`

`claw-local` - моя робоча гілка для власної версії Claw Code.

У ній живуть:

- CLI i18n;
- plugin i18n;
- local model aliases;
- REPL recoverable errors;
- plugin tooling;
- agents;
- local docs/scripts;
- майбутні cloud model launchers;
- майбутні plugins.

## Чому не працюємо прямо в main

`main` має бути чистішим sync-шаром для `upstream/main`.
`claw-local` ізольовано зберігає мої доробки, тому upstream можна підтягувати в `main`, а потім контрольовано merge-ити в робочу гілку.

Так легше:

- бачити різницю між upstream і локальними змінами;
- уникати повторних rebase-конфліктів;
- пушити робочу гілку без ризику зламати sync-гілку;
- відкладати локальні експерименти поза `main`.

## Основні команди

Daily work:

```powershell
git checkout claw-local
git pull
git status
```

Commit:

```powershell
git add .
git commit -m "..."
git push
```

Check updates:

```powershell
scripts/check-upstream-updates.ps1
```

Sync upstream:

```powershell
git checkout main
git fetch upstream
git merge --ff-only upstream/main
git push origin main
```

Merge to local:

```powershell
git checkout claw-local
git merge main
git push
```

## AI prompt: check fork sync state

```text
Перевір стан fork sync для Claw Code.

Працюємо у:
C:\Tools\claw-code\source

Не роби merge, rebase, push, reset або clean.
Тільки перевірка і звіт.

Перевір:
1. git status
2. git branch -vv
3. git remote -v
4. active rebase/merge/cherry-pick/bisect
5. чи current branch `main` або `claw-local`
6. git fetch upstream
7. git fetch origin
8. upstream unique commits щодо main
9. local main unique commits щодо upstream
10. commits in claw-local not in main
11. commits in main not in claw-local

Поверни звіт:
- current branch
- working tree clean yes/no
- active git operation yes/no
- upstream updates yes/no
- main needs update yes/no
- claw-local needs merge from main yes/no
- next safe command
```

## AI prompt: perform safe sync

```text
Виконай безпечну синхронізацію fork workflow для Claw Code.

Працюємо у:
C:\Tools\claw-code\source

Гілки:
- `main` = sync з upstream/main
- `claw-local` = моя робоча гілка

Правила:
- Не роби rebase.
- Не роби reset --hard.
- Не роби clean.
- Не роби push --force.
- Якщо є dirty working tree - зупинись.
- Якщо active rebase/merge/cherry-pick - зупинись.
- Якщо conflict - не коміть автоматично, дай звіт.

Кроки:
1. git checkout main
2. git fetch upstream
3. git merge --ff-only upstream/main
4. git push origin main
5. git checkout claw-local
6. git pull
7. git merge main
8. Якщо merge без конфліктів - git push
9. Якщо merge з конфліктами - показати файли і зупинитись.

Фінальний звіт:
- main updated yes/no
- claw-local updated yes/no
- conflicts yes/no
- push performed yes/no
- next step
```

## AI prompt: resolve merge conflicts

```text
Виріши merge conflicts після `git merge main` у гілці `claw-local`.

Працюємо у:
C:\Tools\claw-code\source

Не роби push.
Не роби reset --hard.
Не роби clean.
Не роби merge --abort без дозволу.
Не роби commit автоматично.

Збережи:
- upstream/main changes;
- мої claw-local changes;
- CLI i18n;
- plugin i18n;
- REPL recoverable errors;
- model aliases;
- agents;
- plugin tooling.

Після виправлення:
- прибери conflict markers;
- git add resolved files;
- не роби commit;
- поверни звіт і команду:
  git commit -m "Merge main into claw-local"
  git push
```
