import spawn from 'cross-spawn';
import chalk from 'chalk';
import ora from 'ora';

/**
 * Executes a command in the shell
 * @param {string} command - Command to execute
 * @param {string[]} args - Command arguments
 * @param {Object} options - Options for spawn
 * @returns {Promise<number>} - Exit code
 */
function executeCommand(command, args = [], options = {}) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      stdio: 'inherit',
      ...options
    });

    child.on('close', (code) => {
      if (code !== 0) {
        reject(new Error(`Command "${command} ${args.join(' ')}" exited with code ${code}`));
        return;
      }
      resolve(code);
    });
  });
}

/**
 * Executes a command with an animated spinner
 * @param {string} text - Text to display
 * @param {Function} fn - Function to execute
 */
async function withSpinner(text, fn) {
  const spinner = ora(text).start();
  try {
    await fn();
    spinner.succeed(chalk.green(`${text} - completed successfully`));
  } catch (error) {
    spinner.fail(chalk.red(`${text} - error: ${error.message}`));
    process.exit(1);
  }
}

/**
 * Displays a section header
 * @param {string} text - Header text
 */
function printHeader(text) {
  console.log('\n' + chalk.bgBlue.white(` ${text} `) + '\n');
}

export { executeCommand, withSpinner, printHeader };
