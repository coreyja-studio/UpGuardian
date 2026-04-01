import { test, expect } from "@playwright/test";
import { test as authedTest } from "./fixtures";

test("unauthenticated home page shows login link", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByText("You are not logged in!")).toBeVisible();
  await expect(page.getByRole("link", { name: "here" })).toBeVisible();
});

authedTest(
  "authenticated home page shows user info and logout",
  async ({ authedPage: page }) => {
    await page.goto("/");
    await expect(page.getByText("Hello,")).toBeVisible();
    await expect(page.getByRole("link", { name: "My Sites" })).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Logout" })
    ).toBeVisible();
  }
);

authedTest("logout redirects to home", async ({ authedPage: page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Logout" }).click();
  await expect(page.getByText("You are not logged in!")).toBeVisible();
});
