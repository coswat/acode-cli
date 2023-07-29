import shellExec from "./shellExec";
const semver = require("semver");

async function updateCli(currentVersion: string | undefined): Promise<void> {
    // get the latest version from npm
    const latestVersion: string | void = await shellExec(
        "npm show @coswat/acode-cli version",
        true,
        true
    );
    // expr; currentVersion < latestVersion
    let result: boolean = semver.lt(currentVersion, latestVersion);
    // check if the above expression was true
    if (!result) {
        console.log(
            "Nothing to update, you are currently using the latest version",
            currentVersion
        );
        process.exit(0);
    }
    // upadte to the latest version
    console.log(`Updating to the latest version ${latestVersion}`);
    await shellExec("npm update -g @coswat/acode-cli");
}

export default updateCli;
