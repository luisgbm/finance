import { commands, call } from './finance';

/// Fetch the full initial payload (accounts, categories, scheduled transactions) once on app
/// start. This desktop build is single-user with no authentication, so there is no login step
/// that would otherwise assemble this data — App bootstraps it directly.
const getInitialData = async () => {
    const { data } = await call(commands.getInitialData());
    return data;
};

export const initialDataService = {
    getInitialData,
};
