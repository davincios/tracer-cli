# Your custom rules!
# /etc/falco/falco_rules.local.yaml

- rule: Run Bioinformatics Executable from /usr/bin
  desc: Detect execution of common bioinformatics executables and SRA tools from /usr/bin.
  condition: evt.type = execve and proc.name in (STAR, bwa, samtools, bcftools, fastqc, hisat2, bowtie2, tophat, cufflinks, htseq)
  output: "Execution of a bioinformatics or SRA executable detected (command=%proc.cmdline, user=%user.name, parent=%proc.pname)"
  priority: NOTICE
  tags: [bioinformatics, sra_tools, user_activity]
  enabled: true






# Tracer CLI tool

- Integrates efficiently with the Tracer backend
- Supports cross-platform compatibility to ensure broad usability across different operating systems commonly used in bioinformatics. 🌍
- Offers a suite of commands that handle key pipeline activities, ensuring a streamlined workflow for end-users. 💼

## Example usage

- tracer --help
- tracer version
- tracer setup <API_KEY>
- tracer start
- tracer log --type warning QC mapping reads GC content below 53% threshold
- tracer tool BWA_MEM2 1.1

## Todos

- ADD this download into a cURL script!!!!
- setup moves the binary automatically to /etc/tracer AND creates a directory there
- do not automatically print the results (api key save and path updated) for tracer setup <API_KEY>
- add to path as "tracer" instead of tracer
- Write tests for to communicate API pipeline steps: initialise pipeline, tool_used, log metric, custom message, alrt and finish pipeline run.
- Make it work locally
- Have CI/CD deployment pipeline
- Write a whole pipeline test

## release mangement

- cargo release patch # for a patch release
- cargo release minor # for a minor release
- cargo release major # for a major release

## Adding to path

- sudo mkdir -p /etc/tracer/
- sudo cp tracer /etc/tracer/tracer
- echo 'export PATH="$PATH:/etc/tracer"' >> ~/.bashrc
- echo 'alias tracer="tracer"' >> ~/.bashrc
- source ~/.bashrc
- tracer --version

## One line installation

curl -sSL https://raw.githubusercontent.com/davincios/tracer-cli/master/install-tracer.sh | bash -s

## How to use on Linux Ubuntu

- check the latest release and copy the link below with the lastest version

```shell
wget https://github.com/davincios/tracer-cli/releases/download/v0.0.22/tracer-x86_64-unknown-linux-gnu.tar.gz
tar -xzf tracer-x86_64-unknown-linux-gnu.tar.gz
./tracer setup <API_KEY>
```
