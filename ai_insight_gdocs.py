import openai
import os
import re
import time
from google.oauth2 import service_account
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

# Set up OpenAI API
openai.api_key = "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"

# Set up Google Docs API
service_account_file = "openai-bridge-to-docs-6718664db8b9.json"
scopes = ["https://www.googleapis.com/auth/documents.readonly"]
credentials = service_account.Credentials.from_service_account_file(service_account_file, scopes=scopes)
docs_service = build("docs", "v1", credentials=credentials)

# Replace the following with your document ID
document_id = "1tz4bypBrP2-vqaP9OV6Zh4wjkrlJoagHAZJ_jUeaZx8"

def read_google_docs_document():
    try:
        doc = docs_service.documents().get(documentId=document_id).execute()
        text = ""
        for content in doc['body']['content']:
            if 'paragraph' in content:
                elements = content['paragraph']['elements']
                for element in elements:
                    if 'textRun' in element:
                        text += element['textRun']['content']
        return text
    except HttpError as error:
        print(f"An error occurred: {error}")
        return None

def parse_document(text):
    sections = re.split(r'\n# (File|Guidance)', text)
    code_files = {}
    guidance = ""
    i = 1
    while i < len(sections):
        if sections[i] == "File":
            file_name = sections[i+1].split("\n", 1)[0].strip()
            code_files[file_name] = sections[i+1].split("\n", 1)[1].strip()
        elif sections[i] == "Guidance":
            guidance = sections[i+1].strip()
        i += 2
    return code_files, guidance

def ask_chat_gpt(prompt):
    while True:
        try:
            response = openai.Completion.create(
                engine="text-davinci-002",
                prompt=prompt,
                max_tokens=800,
                n=1,
                stop=None,
                temperature=0.5,
            )
            return response.choices[0].text.strip()
        except openai.error.OpenAIError as error:
            print(f"An error occurred: {error}")
            if error.args and 'rate limited' in error.args[0]:
                retry_after = int(error.headers.get('retry-after', 60))
                print(f"Rate limited. Retrying in {retry_after} seconds.")
                time.sleep(retry_after)
            else:
                print("Unable to retrieve response. Exiting.")
                return None

def main():
    document_text = read_google_docs_document()
    if document_text:
        code_files, guidance = parse_document(document_text)
        print(f"Code files: {code_files}\n")
        print(f"Guidance: {guidance}\n")
        question = input("Ask a question based on the document: ")
        prompt = f"{document_text}\n{question}"
        response = ask_chat_gpt(prompt)
        print(f"AI response: {response}")
    else:
        print("Unable to read the document.")

if __name__ == "__main__":
    main()
