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

### GitHub Actions Format

```yaml
- name: Generate MD Table
  id: generate-md-table
  uses: Locatarr/markdown-table-generator@<version> # Specific release version tag to pull from
  with:
    file-path: ${{ github.workspace }}/my-json.json
```

When run the action will generate a file that will live for the rest of the job at a random location on disk.
To obtain the file name, use the `output-file` step output.
(Ex. `cat ${{ steps.generate-md-table.outputs.output-file }}`)

### CLI

#### Generate from a file

```bash
$ markdown-table-generator myfile.json
| **Application** | **Description** | **Github** | **Reddit** |
|-|-|-|-|
| AppName | App Description | GitHub Link | Subreddit |
...
```

#### Generate from standard input

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
