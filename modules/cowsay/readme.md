# [cowsay](https://en.wikipedia.org/wiki/Cowsay)

The most important command

## Variations

To see an example of every cow. (Example uses `fortune`)

```bash
for f in /usr/share/cowsay/cows/*; do fortune | cowsay -f $f; done | less
```

Random cow, and figlet

```bash
fortune | figlet | cowsay -n -f $(ls /usr/share/cowsay/cows/ | shuf -n1)
```
