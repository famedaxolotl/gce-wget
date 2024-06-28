# This is a work in progress

## Wget CLI to get past papers from papers.gceguide.com

### Intended command structure:
```bash
[SUBJECT_CODE] -t [LIST_OF_PAPER_TYPES] -c [LIST_OF_PAPER_CODES] [LIST_OF_YEARS]
```
Use `--help` to see more details.

### To-do (roadmap):
- Extract argument parser logic
- Create function which formats the regex
- Run pass the regex onto wget
- Find a solution to allow wget to accept multiple urls

