import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router';
import './FileBrowserComponent.css'

export function FileBrowserComponent() {
  let [isLoading, setIsLoading] = useState(true);
  let [dirEntries, setDirEntries] = useState<DirEntryInfo[]>([]);
  let [breadcrumbs, setBreadcrumbs] = useState<Breadcrumb[]>([]);
  const navigate = useNavigate();

  function handleDirClick(path: string) {
    console.log("DIR HANDLER");
    fetchDirEntries(path).then((response: ListDirResponse) => {
      setDirEntries(response.entries);
      setBreadcrumbs(response.breadcrumbs);
    });
  }

  function handleFileClick(path: string) {
    console.log("FILE HANDLER");
    navigate(`/table?path=${path}`);
  }

  useEffect(() => {
    if (isLoading) {
      fetchDirEntries("/").then((response: ListDirResponse) => {
        setIsLoading(false);
        setDirEntries(response.entries);
        setBreadcrumbs(response.breadcrumbs);
      });
    }
  });

  return (
    <>
      { isLoading
      ? (<div>Loading...</div>)
      : (<>
          <h1>File browser</h1>
          <Breadcrumbs breadcrumbs={breadcrumbs} entryClickedHandler={handleDirClick} />
          <DirList entries={dirEntries} dirClickedHandler={handleDirClick} fileClickedHandler={handleFileClick} />
          <a href="/table">Table</a>
        </>)
      }
    </>
  );
}

interface BreadcrumbsProps {
  breadcrumbs: Breadcrumb[];
  entryClickedHandler(e: string):void;
}

function Breadcrumbs({breadcrumbs, entryClickedHandler}: BreadcrumbsProps) {
  let breadcrumb_links = breadcrumbs.map(b =>
    <li key={b.path}><a onClick={()=>entryClickedHandler(b.path)}>{b.title}</a></li>
  );
  return (
    <nav aria-label="breadcrumb">
      <ol className="breadcrumb">
        {breadcrumb_links}
      </ol>
    </nav>
  );
}

interface DirListProps {
  entries: DirEntryInfo[];
  dirClickedHandler(e: string):void;
  fileClickedHandler(e: string):void;
}

function DirList({entries, dirClickedHandler, fileClickedHandler}: DirListProps) {
    let list = entries.map(entry =>
        <li className={entry.is_file?"file-entry":"dir-entry"}
          key={entry.path}
          onDoubleClick={entry.is_file ? (()=>fileClickedHandler(entry.path)) : (()=> dirClickedHandler(entry.path))}
        >{entry.path}</li>
    );
    return (<ul className="dirs" >{list}</ul>);
}

interface ListDirResponse {
  breadcrumbs: Breadcrumb[];
  entries: DirEntryInfo[];
}

interface Breadcrumb {
  title: string;
  path: string;
}

interface DirEntryInfo {
  path: string;
  is_file: boolean;
}

async function fetchDirEntries(path: string): Promise<ListDirResponse> {
  const response = await fetch(`http://localhost:8080/api/dir/list/${path}`);
  if (!response.ok) {
    throw new Error('Network response was not ok');
  }
  const data: ListDirResponse = await response.json(); 
  return data;
}