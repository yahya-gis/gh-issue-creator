# GitHub Issue Creator

GitHub Issue Creator is a Rust application that simplifies the process of creating GitHub issues from a JSON file and adding them into a GitHub project board.

## Table of Contents

- [GitHub Issue Creator](#github-issue-creator)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Configuration](#configuration)
  - [Usage](#usage)

## Prerequisites

Before using this application, ensure that you have the following prerequisites:

- [Rust](https://www.rust-lang.org/) installed on your system.
- A valid GitHub account and a GitHub repository where you want to create issues.
- A GitHub Personal Access Token with the necessary permissions (repo, project) saved in a `.env` file (See the [Configuration](#configuration) section).
- A valid JSON file containing an array of objects, each representing an issue you want to create.

## Installation

1. **Clone the Repository**: Start by cloning this repository to your local machine:

```bash
git clone https://github.com/olisystems/gh-issue-creator.git
cd gh-issue-creator
```

## Configuration
Configure the application by creating a .env file in the root of the project directory with the following variables:

```bash
GITHUB_ACCESS_TOKEN=your_github_personal_access_token
REPO_OWNER=your_github_username_or_organization
REPO_NAME=your_repository_name
JSON_FILE_NAME=path/to/your/data.json
```

Replace the placeholders with your GitHub Personal Access Token, GitHub repository owner, repository name, and the path to your JSON file.

## Usage
**Configure `.env` File:** Ensure that you have configured the .env file as described in the Configuration section.

**Prepare JSON Data:** Your JSON file should contain an array of objects, where each object represents an issue. For example:

```json
[
  {
    "title": "Sample Issue 1",
    "description": "Description of issue 1"
  },
  {
    "title": "Sample Issue 2",
    "description": "Description of issue 2"
  }
]
```