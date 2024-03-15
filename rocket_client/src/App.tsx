import { useEffect } from "react"

function App() {
  useEffect(()=> {
    // (async ()=> {
    //   const response = await fetch(`http://127.0.0.1:8000/todo`);
    //   console.log(await response.json());
      
    // })()
  })
  return (
    <h1 className="text-center">
      Hello World
    </h1>
  )
}

export default App
