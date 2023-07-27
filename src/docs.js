const os = require("os");
const shellExec = require("./shellExec.js");
require("dotenv").config();

// get the os name
const platform = os.platform();
// plugin docs url
const docsUrl = process.env.DOCS_URL;

// main function docs , to fetch the appropriate command for the os and to executing them
function docs() {
  if (platform === "win32") {
    openDocs(`start ${docsUrl}`);
  } else if (platform === "darwin") {
    openDocs(`open ${docsUrl}`);
  } else {
    openDocs(`xdg-open ${docsUrl}`);
  }
}

// open plugin docs url in web
function openDocs(command) {
  console.log(`Opening ${docsUrl}`);
  shellExec(command);
}

// export the docs function
module.exports = docs;
