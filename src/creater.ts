const inquirer = require("inquirer");
const Listr = require("listr");
const prompts = require("./createrPrompts");
import shellExec from "./shellExec";
const fs = require("fs");
require("dotenv").config();

// interface for Prompts
interface Prompts {
  projectName: string;
  language: "JavaScript" | "TypeScript";
  usePrettier: boolean;
  useGit: boolean;
  installDep: boolean;
}

// interface for Answers
interface Answers {
  projectName: string;
  language: string;
  usePrettier: boolean;
  useGit: boolean;
  installDep: boolean;
}

// Acode plugin url ( Js version )
const jsPlugin: string | undefined = process.env.JS_TEMPLATE_URL;
// Acode plugin url ( Ts version )
const tsPlugin: string | undefined = process.env.TS_TEMPLATE_URL;

// Main function
async function createAcodePlugin(): Promise<void> {
  // questions to ask
  const questions: Prompts = prompts;
  try {
    const answers: Answers = await inquirer.prompt(questions);
    const { projectName, language, usePrettier, useGit, installDep } = answers;
    // current directory
    const currentDir: string = process.cwd();
    // tasks to run
    const tasks = new Listr([
      {
        title: "Cloning plugin template",
        task: () => {
          process.chdir(currentDir);
          if (language === "TypeScript") {
            shellExec(`git clone ${tsPlugin} ${projectName}`);
          } else {
            shellExec(`git clone ${jsPlugin} ${projectName}`);
          }
        },
      },
      {
        title: "CleanUps",
        task: () => {
          process.chdir(currentDir + "/" + projectName);
          shellExec("rm -rf .git");
        },
      },
      {
        title: "Configuring Prettier",
        task: async () => {
          if (usePrettier) {
            await addPrettier();
          }
        },
      },
      {
        title: "Initializing Git repository",
        task: () => {
          if (useGit) {
            shellExec(
              'git init && git add . && git commit -m "Initial Commit"'
            );
          }
        },
      },
      {
        title: "Installing npm dependencies",
        task: () => {
          if (installDep) {
            shellExec("npm install");
          }
        },
      },
    ]);
    // Creating template messgae
    console.log(`Creating Acode plugin '${projectName}' with ${language}...`);
    // running all the tasks
    await tasks.run();
    // Success message
    console.log(
      `Acode plugin '${projectName}' with ${language} successfully created!`
    );
  } catch (error: any) {
    // Error handling
    console.error("Error occurred:", error.message);
    process.exit(1);
  }
}
// add prettier to package.json
async function addPrettier(): Promise<void> {
  // locate the package.json path
  const packageJsonPath: string = process.cwd() + "/package.json";
  // require the file
  const packageJson: any = require(packageJsonPath);
  // add prettier package as a dev dependency
  packageJson.devDependencies = {
    ...packageJson.devDependencies,
    prettier: "3.0.0",
  };
  // json stringify the updated variable
  const updatedPackageJson: string = JSON.stringify(packageJson, null, 2);
  // update the original package.json file
  fs.writeFileSync(packageJsonPath, updatedPackageJson, "utf8");
}
// export the creator function
module.exports = createAcodePlugin;
