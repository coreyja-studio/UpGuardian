import { test as base, expect } from "@playwright/test";

type TestFixtures = {
  authedPage: Awaited<ReturnType<typeof createAuthedPage>>;
};

async function createAuthedPage(
  page: Awaited<ReturnType<(typeof base)["extend"]>["prototype"]["page"]> &
    import("@playwright/test").Page,
  baseURL: string
) {
  // Reset DB state
  await page.request.post(`${baseURL}/test/reset`);

  // Create test user and session via the test-only route
  const response = await page.request.post(`${baseURL}/test/login`);
  expect(response.ok()).toBeTruthy();

  return page;
}

export const test = base.extend<TestFixtures>({
  authedPage: async ({ page, baseURL }, use) => {
    const authed = await createAuthedPage(page, baseURL!);
    await use(authed);
  },
});

export { expect } from "@playwright/test";
