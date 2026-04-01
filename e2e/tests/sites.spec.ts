import { test, expect } from "./fixtures";

test("sites list is empty initially", async ({ authedPage: page }) => {
  await page.goto("/my/sites");
  await expect(page.getByText("My Sites")).toBeVisible();
  await expect(page.getByRole("link", { name: "Create a new site" })).toBeVisible();
});

test("create a site and see it listed", async ({ authedPage: page }) => {
  await page.goto("/my/sites/new");
  await expect(page.getByText("New Site")).toBeVisible();

  await page.getByLabel("Name").fill("Example Site");
  await page.getByLabel("Domain").fill("example.com");
  await page.getByLabel("Description").fill("A test site");
  await page.getByRole("button", { name: "Create" }).click();

  // Should redirect to site detail page
  await expect(page.getByRole("heading", { name: "Example Site" })).toBeVisible();
  await expect(page.getByText("A test site")).toBeVisible();
  await expect(page.getByRole("link", { name: "Visit Site" })).toBeVisible();

  // Go back to list and verify it shows up
  await page.goto("/my/sites");
  await expect(page.getByRole("link", { name: "Example Site", exact: true })).toBeVisible();
});

test("site detail page shows pages section", async ({ authedPage: page }) => {
  // Create a site first
  await page.goto("/my/sites/new");
  await page.getByLabel("Name").fill("My App");
  await page.getByLabel("Domain").fill("myapp.dev");
  await page.getByRole("button", { name: "Create" }).click();

  await expect(page.getByRole("heading", { name: "Pages" })).toBeVisible();
  await expect(page.getByRole("link", { name: "Create a new page" })).toBeVisible();
});
