import os
import requests
import openai
import PySimpleGUI as sg
from github import Github
from datetime import datetime

# Set your API keys
OPENAI_API_KEY = "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"
GITHUB_API_KEY = "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"

# Configure OpenAI
openai.api_key = OPENAI_API_KEY

# Configure GitHub
g = Github(GITHUB_API_KEY)

# Get the repository
repo = g.get_repo("Tamok/incremental_unfolding_game")

# Get repository file contents
repo_contents = repo.get_contents("")

# Function to recursively get files and folders in the repository
def get_files_recursive(contents, path=""):
    result = ""
    for content_file in contents:
        if content_file.type == "dir":
            result += get_files_recursive(repo.get_contents(content_file.path), path + content_file.name + "/")
        else:
            result += path + content_file.name + "\n"
    return result

# Get repository file contents and process folders
repo_contents = get_files_recursive(repo.get_contents(""))

# Function to ask GPT-4 a question with an optional context
def ask_gpt_4(question, context=None):
    prompt = question
    if context:
        prompt = f"{context}\n{question}"
    
    response = openai.Completion.create(
        engine="text-davinci-002",
        prompt=prompt,
        max_tokens=150,
        n=1,
        stop=None,
        temperature=0.5,
    )
    return response.choices[0].text.strip()

# PySimpleGUI layout
layout = [
    [sg.Text("Enter your question:")],
    [sg.InputText("", key="question")],
    [sg.Button("Ask GPT-4"), sg.Button("Exit")],
    [sg.ProgressBar(1, orientation='h', size=(20, 20), key='progress')],
    [sg.Output(size=(80, 20), key="output")],
]

# Create the window
window = sg.Window("GPT-4 Repository Explorer", layout)

# Event loop
while True:
    event, values = window.read()
    
    if event == sg.WIN_CLOSED or event == "Exit":
        break
    
    # Ask GPT-4 a question about the repository when the "Ask GPT-4" button is clicked
    if event == "Ask GPT-4":
        try:
            question = values["question"]
            context = f"Here is a list of files in the repository 'incremental_unfolding_game':\n\n"
            context += repo_contents
            
            # Update the progress bar while GPT-4 processes the question
            progress_bar = window['progress']
            for i in range(1, 101):
                progress_bar.UpdateBar(i)
                sg.time.sleep(0.01)
            
            answer = ask_gpt_4(question, context)
            
            # Clear the input field
            window["question"].update("")
            
            # Timestamp and separate answers
            timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            window["output"].update(f"{timestamp} - Question: {question}\n{timestamp} - Answer: {answer}\n\n", append=True)
            
            # Reset the progress bar
            progress_bar.UpdateBar(0)
        except Exception as e:
            window["output"].update(f"An error occurred: {str(e)}")

window.close()