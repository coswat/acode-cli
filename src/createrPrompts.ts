const chalk = require("chalk");
// promts for 'create' command
module.exports = [
  {
    type: "input",
    name: "projectName",
    message: "Enter a name for your Acode plugin:",
    default: "test-plugin",
  },
  {
    type: "list",
    name: "language",
    message: "Choose a language:",
    choices: [chalk.yellow("JavaScript"), chalk.cyan("TypeScript")],
  },
  {
    type: "confirm",
    name: "usePrettier",
    message: "Do you want to use Prettier for code formatting?",
    default: true,
  },
  {
    type: "confirm",
    name: "useGit",
    message: "Initialize a Git repository?",
    default: true,
  },
  {
    type: "confirm",
    name: "installDep",
    message: "Install npm dependencies?",
    default: true,
  },
];
