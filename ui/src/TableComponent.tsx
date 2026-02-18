import { useEffect, useRef, useState } from 'react'

import 'react-data-grid/lib/styles.css';
import { DataGrid, type Column, type ColumnOrColumnGroup, type ColumnWidths } from 'react-data-grid';
import { useSearchParams } from 'react-router';

export function TableComponent() {
  let [isLoading, setIsLoading] = useState(true);
  let request_in_progress = useRef(false);
  let [cols, setCols] = useState<readonly ColumnOrColumnGroup<object, unknown>[]>([]);
  let [rows, setRows] = useState<readonly object[]>([]);
  const [queryParams] = useSearchParams();
  console.log(queryParams);

  useEffect(() => {
    if (isLoading) {
      if (!request_in_progress.current) {
        request_in_progress.current = true;
        fetchCsvTable(queryParams.get("path")!).then((response: CsvTable) => {
          setIsLoading(false);
          setCols(response.columns);
          setRows(response.rows);
          request_in_progress.current = false;
        });
      }
    }
  });

  return (
    <>
    {isLoading ? (
      <div>Loading...</div>
    ):(
      <DataGrid
        columns={cols}
        rows={rows}
        defaultColumnOptions={{/*minWidth: 100,*/ resizable: true, sortable: true, draggable: true}}
      />
    )}
    </>
  );
}

interface ColumnInfo {
  key: string,
  name: string,
  comment?: string;
}

interface CsvTable {
    columns: readonly ColumnInfo[];
    rows: readonly object[];
}

async function fetchCsvTable(path: string): Promise<CsvTable> {
  console.log(`PATH: http://localhost:8080/api/csv-table/${path}`);
  const response = await fetch(`http://localhost:8080/api/csv-table/${path}`);
  if (!response.ok) {
    throw new Error('Network response was not ok');
  }
  const data: CsvTable = await response.json(); 
  return data;
}
