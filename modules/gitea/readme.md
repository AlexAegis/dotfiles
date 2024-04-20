# [gitea](https://about.gitea.com/)

Private git hosting platform

> [github](https://github.com/go-gitea/gitea)

## Setup

> [Gitea/Postgres Docs](https://docs.gitea.com/installation/database-prep#postgresql)

After installing this module, [postgres](../postgresql/) should also be already
installed. Create a table in 

```sh
sudo -u postgres psql
CREATE ROLE gitea WITH LOGIN PASSWORD 'YOUR_PASSWORD';
```
