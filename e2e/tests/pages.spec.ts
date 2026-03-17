import { test, expect } from "./fixtures";

test("create a page within a site", async ({ authedPage: page }) => {
  // Create a site first
  await page.goto("/my/sites/new");
  await page.getByLabel("Name").fill("Page Test Site");
  await page.getByLabel("Domain").fill("pagetest.dev");
  await page.getByRole("button", { name: "Create" }).click();

  // Click "Create a new page"
  await page.getByRole("link", { name: "Create a new page" }).click();
  await expect(page.getByText("New Page")).toBeVisible();

  await page.getByLabel("Path").fill("/health");
  await page.getByLabel("Name").fill("Health Check");
  await page.getByRole("button", { name: "Create" }).click();

  // Should redirect back to site detail showing the new page
  await expect(page.getByRole("link", { name: /Health Check/ })).toBeVisible();
});
