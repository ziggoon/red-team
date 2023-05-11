import re
import os

file_path = input("enter the file path: ")
with open(file_path, 'r') as file:
    blob_text = file.read()

pattern = r'user:\[([^\]]+)\]'

matches = re.findall(pattern, blob_text)

usernames = [match.strip() for match in matches]

print("usernames:")
for username in usernames:
    print(username)

os.makedirs("output", exist_ok=True)
outfile = os.path.join("output", "domain_users")

with open(outfile, 'w') as output_file:
    for username in usernames:
        output_file.write(username + '\n')

print("users written to file in output/domain_users")
