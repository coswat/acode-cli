const os = require("os");
import shellExec from "./shellExec";
const path = require("path");

const dotenvPath: string = path.resolve(__dirname, "../config/", ".env");

require("dotenv").config({ path: dotenvPath });

// get the os name
const platform: string = os.platform();
// plugin docs url
const docsUrl: string | undefined = process.env.DOCS_URL;

// main function docs , to fetch the appropriate command for the os and to executing them
async function docs(): Promise<void> {
  if (platform === "win32") {
    await openDocs(`start ${docsUrl}`);
  } else if (platform === "darwin") {
    await openDocs(`open ${docsUrl}`);
  } else {
    await openDocs(`xdg-open ${docsUrl}`);
  }
}

// open plugin docs url in web
async function openDocs(command: string): Promise<void> {
  console.log(`Opening ${docsUrl}`);
  await shellExec(command);
}

// export the docs function
module.exports = docs;
