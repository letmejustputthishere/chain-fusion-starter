import { isValidName } from "ethers";
import { createPublicClient, http } from 'viem';
import { addEnsContracts } from '@ensdomains/ensjs';
import { getOwner } from '@ensdomains/ensjs/public';
import { mainnet, sepolia, optimism } from 'viem/chains';

export const validateEns = (ens: string, setEnsError?: Function): boolean => {
    let result: boolean = true;

    if (!isValidName(ens)) {
        result = false;
    }

    if (!ens.includes(".") || !ens.endsWith(".eth") || ens.split(".")[0].length < 3) {
        result = false;
    }

    if (!result && setEnsError) {
        setEnsError("Invalid ENS name");
    }

    return result;
}

const validateRpc = (rpc: string, setRpcError: Function): boolean => {
    try {

        // If RPC is not provided, means it will be added in .env later.
        // So, no need to validate it
        if (!rpc.length) return true;

        const prefixCheck = rpc.startsWith("http://") || rpc.startsWith("https://");
        const contentCheck = rpc.split("//")[1].length;

        if (!prefixCheck || !contentCheck) {
            setRpcError("Invalid RPC endpoint");
            return false;
        }

        return true;

    } catch (error) {
        setRpcError("Invalid RPC endpoint");
        return false;
    }
}

const validateUrl = (url: string, setUrlError: Function): boolean => {
    try {

        const prefixCheck = url.startsWith("http://") || url.startsWith("https://");
        const contentCheck = url.split("//")[1].length;

        if (!prefixCheck || !contentCheck) {
            setUrlError("Invalid URL endpoint");
            return false;
        }

        return true;

    } catch (error) {
        setUrlError("Invalid URL endpoint");
        return false;
    }
}

export const areAllPropertiesValid = async (
    ens: string,
    setEnsError: Function,
    rpc: string,
    setRpcError: Function,
    url: string,
    setUrlError: Function,
): Promise<boolean> => {
    const isEnsValid = await validateEns(ens, setEnsError);
    const isRpcValid = validateRpc(rpc, setRpcError);
    const isUrlValid = validateUrl(url, setUrlError);
    return (isEnsValid && isRpcValid && isUrlValid);
}

export const isAccountOwnerOfEnsName = async (
    ensName: string,
    account: string,
    setEnsError: Function,
    chainId: number,
): Promise<boolean> => {

    try {

        const chain = chainId === 1 ? mainnet : (chainId === 10 ? optimism : sepolia);

        const client = createPublicClient({
            chain: addEnsContracts(chain),
            transport: http(),
        });

        const result = await getOwner(client, { name: ensName });

        if (result && result.owner && result.owner.toLowerCase() === account.toLowerCase()) {
            console.log("You are not the owner of ens name : ", result)
            return true;
        }

        setEnsError("You are not the owner of this name");
        return false;

    } catch (error) {
        console.log("Error in validating ens name : ", error);
        setEnsError("You are not the owner of this name");
        return false;
    }

};