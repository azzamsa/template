import { test, expect } from '@playwright/test'

test('index page has expected h1', async ({ page }) => {
  // Go to homepage
  await page.goto('/')

  const locator = page.locator('h1')
  await expect(locator).toContainText('Hi')
})
