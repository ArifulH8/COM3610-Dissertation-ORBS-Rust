import { expect, test } from 'vitest';
import { weird_function } from './index';

test('run weird_function with 8 ', () => {
  expect(weird_function(8)).toBe(64);
});