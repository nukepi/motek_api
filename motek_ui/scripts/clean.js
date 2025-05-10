import { executeCommand, withSpinner, printHeader } from './utils.js';
import { join } from 'path';

async function clean() {
  printHeader('Cleaning Motek UI project');

  await withSpinner('Cleaning Flutter project', async () => {
    await executeCommand('flutter', ['clean']);
  });

  await withSpinner('Cleaning Rust dependencies', async () => {
    const rustDir = join(process.cwd(), 'rust');
    await executeCommand('cargo', ['clean'], { cwd: rustDir });
  });

  console.log('\nProject cleaned successfully! 🧹');
}

clean().catch(err => {
  console.error('\nError while cleaning project:', err);
  process.exit(1);
});
