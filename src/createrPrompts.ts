const chalk = require("chalk");

// maximum plugin id length
const pluginIdRegex: RegExp = /^[a-zA-Z][a-zA-Z0-9._-]{3,49}$/;
// maximum and minimum plugin price
const minPluginPrice: number = 10;
const maxPluginPrice: number = 10_000;

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
        type: "input",
        name: "pluginId",
        message: "Enter id for your Acode Plugin:",
        default: "acode-plugin-test12",
        validate: function (input: string) {
            const done: any = this.async();
            setTimeout(function () {
                if (!pluginIdRegex.test(input)) {
                    // Pass the return value in the done callback
                    done(
                        `Id should start with an alphabet, should be of length 4-50 and should contain only alphanumeric characters, dot, - and underscore.`
                    );
                    return;
                }
                // Pass the return value in the done callback
                done(null, true);
            }, 3000);
        },
    },
    {
        type: "number",
        name: "pluginPrice",
        message:
            "Enter price for your Acode Plugin in INR, if it's free then leave it on default value:",
        default: 0,
        validate: function (input: number) {
            const done: any = this.async();
            setTimeout(function () {
                // validating plugin price
                const isInRange: boolean =
                    input === 0 ||
                    (input > minPluginPrice && input < maxPluginPrice);
                if (!isInRange) {
                    // Pass the return value in the done callback
                    done(
                        `Price should be between INR ${minPluginPrice} and INR ${maxPluginPrice}`
                    );
                    return;
                }
                // Pass the return value in the done callback
                done(null, true);
            }, 3000);
        },
    },
    {
        type: "input",
        name: "authorName",
        message: "Enter the Name of Plugin developer:",
        default: "",
    },
    {
        type: "input",
        name: "authorEmail",
        message: "Enter the Email of Plugin developer:",
        default: "",
    },
    {
        type: "input",
        name: "authorGitHubUname",
        message:
            "Enter the Github username of Plugin developer!:",
        default: "",
    },
    {
        type: "confirm",
        name: "installDep",
        message: "Install npm dependencies?",
        default: true,
    },
];
