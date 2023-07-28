const path = require("path");

// Load the .env file from config dir
function loadEnv(): void {
  // absolute dotenv path
  const dotenvPath: string = path.resolve(__dirname, "../config/", ".env");
  // require the dotenv file
  require("dotenv").config({ path: dotenvPath });
}

module.exports = {
  loadEnv,
};
