import { useState } from "react"
import { ThemeProvider } from "@/components/theme-provider"
import { Button } from "@/components/ui/button"
import { testDatabase } from "@/lib/tauri"

function App() {
  const [greetMsg, setGreetMsg] = useState("")
  const [dbStatus, setDbStatus] = useState("")

  // Test database connection via Tauri command
  const handleTestDatabase = async () => {
    try {
      const result = await testDatabase()
      setDbStatus(`Database connected: ${result}`)
    } catch (error) {
      setDbStatus(`Database error: ${error}`)
    }
  }

  return (
    <ThemeProvider defaultTheme="dark" storageKey="yambam-ui-theme">
      <div className="min-h-screen bg-background text-foreground">
        <div className="container mx-auto p-8">
          {/* Main heading */}
          <h1 className="text-4xl font-bold mb-8">
            Hello YamBam
          </h1>

          {/* Test buttons */}
          <div className="space-y-4">
            <Button
              onClick={() => setGreetMsg("Welcome to YamBam!")}
              variant="default"
            >
              Test UI
            </Button>

            <Button
              onClick={handleTestDatabase}
              variant="secondary"
            >
              Test Database
            </Button>

            {/* Status messages */}
            {greetMsg && (
              <p className="text-lg text-muted-foreground">{greetMsg}</p>
            )}

            {dbStatus && (
              <p className="text-lg font-mono text-sm">{dbStatus}</p>
            )}
          </div>
        </div>
      </div>
    </ThemeProvider>
  )
}

export default App
