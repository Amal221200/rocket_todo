import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import TodoForm from './components/Popup/TodoForm.tsx'
import { ToastContainer } from 'react-toastify'
import 'react-toastify/ReactToastify.css';
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
    <TodoForm />
    <ToastContainer theme='dark' autoClose={3000} />
  </React.StrictMode>
)
