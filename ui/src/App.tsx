import { createBrowserRouter, RouterProvider } from "react-router";
import './App.css'
import { TableComponent } from './TableComponent';
import { FileBrowserComponent } from './FileBrowserComponent';

let router = createBrowserRouter([
  {
    path: "/",
    Component: FileBrowserComponent,
  },
  {
    path: "/table",
    Component: TableComponent,
  },
]);

function App() {
  return (<RouterProvider router={router} />);
}

export default App