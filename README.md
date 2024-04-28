# Tracer CLI tool

- Integrates efficiently with the Tracer backend
- Supports cross-platform compatibility to ensure broad usability across different operating systems commonly used in bioinformatics. 🌍
- Offers a suite of commands that handle key pipeline activities, ensuring a streamlined workflow for end-users. 💼

## How to use on Linux Ubuntu

- wget https://github.com/davincios/tracer-cli/releases/download/v0.0.14/tracer_cli-x86_64-unknown-linux-gnu.tar.gz
- tar -xzf tracer_cli-x86_64-unknown-linux-gnu.tar.gz

## Adding to path

- sudo mkdir -p /etc/tracer/
- sudo cp tracer_cli /etc/tracer/tracer_cli
- echo 'export PATH="$PATH:/etc/tracer"' >> ~/.bashrc
- echo 'alias tracer="tracer_cli"' >> ~/.bashrc
- source ~/.bashrc
- tracer --version

## Example usage

- tracer --help
- tracer --version
- tracer setup <API_KEY_HERE>
- tracer start
- tracer log --type warning QC mapping reads GC content below 53% threshold
- tracer tool BWA_MEM2 1.1

## Todos

- setup moves the binary automatically to /etc/tracer AND creates a directory there
- do not automatically print the results (api key save and path updated) for tracer setup <API_KEY>
- add to path as "tracer" instead of tracer_cli
- Write tests for to communicate API pipeline steps: initialise pipeline, tool_used, log metric, custom message, alrt and finish pipeline run.
- Make it work locally
- Have CI/CD deployment pipeline
- Write a whole pipeline test
