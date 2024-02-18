 import { expect, test } from 'vitest'
import { hello_name } from './index'

test('run hello_name with Ariful ', () => {
  expect(hello_name("Ariful")).toBe(`Hello World and hello Ariful`)
})