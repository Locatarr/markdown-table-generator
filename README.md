# Markdown Table Generator

Generate markdown tables from a JSON definition for publication on [locatarr.github.io](https://locatar.github.io).

## Usage

### JSON file format

```jsonc
{
    "applications": [
        {
            "name": "AppName",
            "description": "App Description",
            "github_slug": "github/link", // github_slug is an optional field
            "subreddit": "r/subreddit" // subreddit is an optional field
        }
    ]
}
```

### Generate from a file

```bash
$ markdown-table-generator myfile.json
| **Application** | **Description** | **Github** | **Reddit** |
|-|-|-|-|
| AppName | App Description | GitHub Link | Subreddit |
...
```

### Generate from standard input

```bash
$ cat myfile.json | markdown-table-generator -
| **Application** | **Description** | **Github** | **Reddit** |
|-|-|-|-|
| AppName | App Description | GitHub Link | Subreddit |
...
```

OR

```bash
$ cat myfile.json | markdown-table-generator
| **Application** | **Description** | **Github** | **Reddit** |
|-|-|-|-|
| AppName | App Description | GitHub Link | Subreddit |
...
```
