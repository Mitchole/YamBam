import { invoke } from "@tauri-apps/api/core"

export async function testDatabase(): Promise<string> {
  try {
    const result = await invoke<string>("test_database")
    return result
  } catch (error) {
    console.error("Database test failed:", error)
    throw error
  }
}
