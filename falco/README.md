# Falco Installation
### Add the falco repository key
```bash
curl -fsSL https://falco.org/repo/falcosecurity-packages.asc | \
sudo gpg --dearmor -o /usr/share/keyrings/falco-archive-keyring.gpg
 ```
 ### Add the falco repository 
```bash
sudo bash -c 'cat << EOF > /etc/apt/sources.list.d/falcosecurity.list
deb [signed-by=/usr/share/keyrings/falco-archive-keyring.gpg] https://download.falco.org/packages/deb stable main
EOF'
```
### Read the repository content 
```bash
sudo apt-get update -y
```


# Summary 
# Stop the Falco service
sudo systemctl stop falco

# Remove Falco
sudo apt-get remove --purge -y falco

# Remove remaining configuration files
sudo rm -rf /etc/falco
sudo rm -rf /var/log/falco
sudo rm -rf /usr/local/bin/falco
sudo rm -rf /usr/src/falco

# Add Falco's repository
curl -s https://falco.org/repo/falcosecurity-packages.asc | sudo apt-key add -
sudo bash -c 'echo "deb https://download.falco.org/packages/deb stable main" > /etc/apt/sources.list.d/falcosecurity.list'

# Update package lists
sudo apt-get update

# Install Falco
sudo apt-get install -y falco

# Start Falco
sudo systemctl start falco

# Enable Falco to start on boot
sudo systemctl enable falco

# Check the status of Falco
sudo systemctl status falco
