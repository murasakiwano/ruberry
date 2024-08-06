import { invoke as invokeRaw } from "@tauri-apps/api/core";

export type Transaction = {
  id: string;
  date: string;
  category: string;
  title: string;
  amount: number;
};

type TauriCommands = {
  get_transactions: {
    returns: Transaction[];
    args: undefined;
  };
  get_expenses: {
    returns: Transaction[];
    args: undefined;
  };
};

export function invoke<T extends keyof TauriCommands>(
  cmd: T,
  args?: TauriCommands[T]["args"],
): Promise<TauriCommands[T]["returns"]> {
  return invokeRaw(cmd, args);
}
