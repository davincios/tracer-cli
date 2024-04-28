# Tracer CLI tool

- Integrates efficiently with the Tracer backend
- Supports cross-platform compatibility to ensure broad usability across different operating systems commonly used in bioinformatics. 🌍
- Offers a suite of commands that handle key pipeline activities, ensuring a streamlined workflow for end-users. 💼

## How to use on Linux Ubuntu

- wget https://github.com/davincios/tracer-cli/releases/download/v0.0.14/tracer_cli-x86_64-unknown-linux-gnu.tar.gz
- tar -xzf tracer_cli-x86_64-unknown-linux-gnu.tar.gz

## Add to path // need to fix

Todos:

- setup moves the cli automatically to temp
- add standard base_url
- add to path as "tracer" instead of tracer_cli

## Example usage

- tracer_cli --help
- tracer_cli --version
- tracer_cli setup <API_KEY_HERE>
- tracer_cli start
- tracer_cli log --type warning QC mapping reads GC content below 53% threshold
- tracer_cli tool BWA_MEM2 1.1

## Todos

- Write tests for to communicate API pipeline steps: initialise pipeline, tool_used, log metric, custom message, alrt and finish pipeline run.
- Make it work locally
- Have CI/CD deployment pipeline
- Write a whole pipeline test
