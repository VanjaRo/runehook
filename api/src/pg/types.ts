export type DbPaginatedResult<T> = {
  total: number;
  results: T[];
};

export type DbCountedResult<T> = T & { total: number };

export type DbRune = {
  id: string;
  number: number;
  name: string;
  spaced_name: string;
  block_height: number;
  tx_index: number;
  tx_id: string;
  divisibility: number;
  premine: string;
  symbol: string;
  terms_amount: string | null;
  terms_cap: string | null;
  terms_height_start: number | null;
  terms_height_end: number | null;
  terms_offset_start: number | null;
  terms_offset_end: number | null;
  turbo: boolean;
  minted: string;
  total_mints: number;
  burned: string;
  total_burns: number;
  timestamp: number;
};
