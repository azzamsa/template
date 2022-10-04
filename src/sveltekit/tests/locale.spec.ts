import { test, expect } from '@playwright/test'

test('test toggle locale button', async ({ page }) => {
  // Go to homepage
  await page.goto('/')

  const locator_en = page.locator('h1')
  await expect(locator_en).toContainText('Hi')

  // Change locale
  await page.locator('button').nth(1).click()

  // Need to use two locator variable, otherwise it fails on chromium test
  const locator_id = page.locator('h1')
  await expect(locator_id).toContainText('Hai')
})
