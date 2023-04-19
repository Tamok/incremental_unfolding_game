import openai
import os
import re
import time
import PySimpleGUI as sg
from github import Github
from github.GithubException import BadCredentialsException, RateLimitExceededException
import json
import toml

# Set up OpenAI API
openai.api_key = "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"

# Set up Github API
g = Github("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")
repo = g.get_repo("Tamok/incremental_unfolding_game")

def get_all_files_in_repo():
    file_contents = {}
    contents = repo.get_contents("")
    while contents:
        file_content = contents.pop(0)
        if file_content.type == "dir":
            contents.extend(repo.get_contents(file_content.path))
        else:
            try:
                decoded_content = file_content.decoded_content.decode()
                if file_content.path.endswith(".rs"):
                    prompt = "# coding: rust\n"
                    prompt += decoded_content
                    decoded_content = prompt
                elif file_content.path.endswith(".json"):
                    decoded_content = json.loads(decoded_content)
                elif file_content.path.endswith(".toml"):
                    decoded_content = toml.loads(decoded_content)
                file_contents[file_content.path] = decoded_content
            except:
                print(f"Unable to read {file_content.path}")
    return file_contents

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
    file_contents = get_all_files_in_repo()
    file_list = list(file_contents.keys())
    layout = [
        [sg.Text("Select a file to ask a question about:")],
        [sg.Listbox(file_list, size=(60, 20), key="-FILE LIST-", enable_events=True)],
        [sg.Text("Ask a question about the selected file:")],
        [sg.Input(key="-QUESTION-", size=(60, 1))],
        [sg.Button("Submit"), sg.Button("Exit")]
    ]
    window = sg.Window("GitHub AI", layout)
    while True:
        event, values = window.read()
        if event == "Exit" or event == sg.WIN_CLOSED:
            break
        elif event == "Submit":
            selected_file = values["-FILE LIST-"][0]
            question = values["-QUESTION-"]
            content = file_contents[selected_file]
            prompt = f"# {selected_file}\n{content}\n{question}"
            response = ask_chat_gpt(prompt)
            sg.popup(f"AI response: {response}")

if __name__ == "__
