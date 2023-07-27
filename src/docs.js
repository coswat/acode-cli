const os = require("os");
const shellExec = require("./shellExec.js");
require("dotenv").config();

// get the os name
const platform = os.platform();
// plugin docs url
const docsUrl = process.env.DOCS_URL;

// main function docs , to fetch the appropriate command for the os and to executing them
async function docs() {
  if (platform === "win32") {
    await openDocs(`start ${docsUrl}`);
  } else if (platform === "darwin") {
    await openDocs(`open ${docsUrl}`);
  } else {
    await openDocs(`xdg-open ${docsUrl}`);
  }
}

// open plugin docs url in web
async function openDocs(command) {
  console.log(`Opening ${docsUrl}`);
  shellExec(command);
}

// export the docs function
module.exports = docs;
