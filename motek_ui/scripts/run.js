import { executeCommand, printHeader } from './utils.js';

async function run() {
  printHeader('Running Motek UI application');
  
  try {
    await executeCommand('flutter', ['run', '-v']);
  } catch (error) {
    console.error('\nError while running application:', error);
    process.exit(1);
  }
}

run();
