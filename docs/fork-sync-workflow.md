# Fork sync workflow

Цей документ описує безпечну схему роботи з fork-гілками Claw Code.
Головна ідея: `main` тримаємо як чисту sync-гілку для `upstream/main`, а щоденну роботу ведемо тільки в `claw-local`.

## Branch roles

- `upstream/main` - оригінальний Claw Code.
- `origin/main` - `main` у моєму fork, який має бути максимально близький до `upstream/main`.
- local `main` - локальна sync-гілка для оновлення з upstream.
- `origin/claw-local` - моя робоча гілка у fork.
- local `claw-local` - моя основна робоча гілка для доробок.

Правила:

- Щоденна робота тільки у `claw-local`.
- `main` не використовувати для локальних доробок.
- `main` оновлювати тільки з `upstream/main`.
- Зміни з `main` підтягувати у `claw-local` через `git merge main`.

## First-time setup

Якщо `claw-local` ще не створена:

```powershell
git checkout main
git pull origin main
git checkout -b claw-local
git push -u origin claw-local
```

Якщо `claw-local` уже існує:

```powershell
git fetch origin
git checkout claw-local
git pull
```

## Daily work

```powershell
git checkout claw-local
git status
git pull
```

Після змін:

```powershell
git add .
git commit -m "..."
git push
```

## Check upstream updates safely

```powershell
scripts/check-upstream-updates.ps1
```

Цей скрипт тільки перевіряє стан:

- не робить merge;
- не робить rebase;
- не робить push;
- не робить reset або clean.

## Update main from upstream

```powershell
git checkout main
git status
git fetch upstream
git merge --ff-only upstream/main
git push origin main
```

Якщо `git merge --ff-only upstream/main` не проходить, не робити `merge --no-ff` автоматично.
Зупинись і перевір, чи `main` не має локальних змін або зайвих комітів.

## Bring upstream changes into claw-local

```powershell
git checkout claw-local
git status
git pull
git merge main
git push
```

Для `claw-local` використовуємо `git merge main`, а не `git rebase upstream/main`.
Так менше шансів повторити конфліктний rebase-лабіринт і легше бачити, де саме upstream зміни увійшли в локальну гілку.

## Conflict handling during merge main

Спочатку перевір стан:

```powershell
git status
```

Після ручного виправлення конфліктів:

```powershell
git add <resolved-files>
git commit -m "Merge main into claw-local"
git push
```

Якщо merge треба скасувати:

```powershell
git merge --abort
```

Роби це тільки якщо зрозумілі наслідки і є явна згода.

## What not to do

- Не робити `git rebase upstream/main` напряму у `claw-local`.
- Не натискати GitHub "Discard commits".
- Не робити `git reset --hard`.
- Не робити `git clean -fdx`.
- Не робити `git push --force` без окремого плану.
- Не синхронізуватися, якщо working tree dirty.
- Не синхронізуватися, якщо active rebase/merge/cherry-pick/bisect.

## Recovery

Якщо випадково запустили rebase:

```powershell
git status
```

Якщо це помилковий rebase і ми не хочемо його продовжувати:

```powershell
git rebase --abort
```

Після цього:

```powershell
git status
git branch -vv
```

Не роби `git rebase --continue`, `git rebase --skip`, `git reset --hard` або `git clean -fdx` без окремого рішення.

## Full manual sync sequence

```powershell
git checkout claw-local
git status
git pull

git checkout main
git status
git fetch upstream
git merge --ff-only upstream/main
git push origin main

git checkout claw-local
git status
git merge main
git push
```

Якщо на будь-якому кроці з'являються conflicts або dirty working tree, зупинись і спочатку розбери стан.
