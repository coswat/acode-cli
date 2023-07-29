const shelljs = require("shelljs");

const { exec } = shelljs;

// function to execute commands in cli
async function shellExec(
    command: string,
    silentMode: boolean = true,
    stdout: boolean = false
): Promise<void | string> {
    const result = exec(command, { silent: silentMode });
    if (result.code !== 0) {
        throw new Error(`Error executing command: ${command}`);
    }
    if (stdout) {
        return result.stdout.trim();
    }
}

// export the shellExec function
export default shellExec;
