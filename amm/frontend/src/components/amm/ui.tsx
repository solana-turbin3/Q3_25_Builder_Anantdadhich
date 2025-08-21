// This file contains only the presentational components for the AMM.
// It receives all its data and functions as props.

const Card = ({ children, title }:any) => (
    <div className="bg-gray-800 rounded-lg shadow-lg p-6 w-full max-w-md">
        <h2 className="text-2xl font-bold text-white mb-4">{title}</h2>
        {children}
    </div>
);

const Input = (props:any) => (
    <input
        className="w-full bg-gray-700 text-white rounded-md p-3 mb-4 focus:outline-none focus:ring-2 focus:ring-purple-500 transition"
        {...props}
    />
);

const Button = ({ children, onClick, isLoading, ...props }:any) => (
    <button
        onClick={onClick}
        disabled={isLoading}
        className="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-4 rounded-md transition disabled:bg-gray-500 disabled:cursor-not-allowed flex justify-center items-center"
        {...props}
    >
        {isLoading ? (
            <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
        ) : (
            children
        )}
    </button>
);

export function AmmUi({
    seed, setSeed,
    mintX, setMintX,
    mintY, setMintY,
    handleInitialize, initializeLoading,
    configPda,
    depositAmount, setDepositAmount,
    handleDeposit, depositLoading,
    swapAmount, setSwapAmount,
    swapDirection, setSwapDirection,
    handleSwap, swapLoading,
    poolState,
    txSignature,
}:any) {
    return (
        <div className="space-y-8">
            <Card title="1. Initialize New Pool">
                <Input placeholder="Enter a random number (seed)" value={seed} onChange={(e:any) => setSeed(e.target.value)} type="number" />
                <Input placeholder="Mint X Public Key" value={mintX} onChange={(e:any) => setMintX(e.target.value)} />
                <Input placeholder="Mint Y Public Key" value={mintY} onChange={(e:any) => setMintY(e.target.value)} />
                <Button onClick={handleInitialize} isLoading={initializeLoading}>Initialize Pool</Button>
            </Card>

            {configPda && (
                <div className="text-center text-green-400">
                    <p>Active Pool Config: {configPda.toBase58()}</p>
                </div>
            )}

            <Card title="2. Pool Actions">
                <div className="mb-6">
                    <h3 className="text-xl font-semibold text-white mb-2">Provide Liquidity</h3>
                    <Input placeholder="LP Amount to Deposit (in smallest unit)" value={depositAmount} onChange={(e:any) => setDepositAmount(e.target.value)} type="number" />
                    <Button onClick={handleDeposit} isLoading={depositLoading}>Deposit</Button>
                </div>
                <div>
                    <h3 className="text-xl font-semibold text-white mb-2">Swap Tokens</h3>
                    <select
                        value={swapDirection}
                        onChange={(e) => setSwapDirection(e.target.value)}
                        className="w-full bg-gray-700 text-white rounded-md p-3 mb-4"
                    >
                        <option value="x_to_y">Swap X for Y</option>
                        <option value="y_to_x">Swap Y for X</option>
                    </select>
                    <Input placeholder="Amount to Swap (in smallest unit)" value={swapAmount} onChange={(e:any) => setSwapAmount(e.target.value)} type="number" />
                    <Button onClick={handleSwap} isLoading={swapLoading}>Swap</Button>
                </div>
            </Card>
            
            {poolState && (
                <Card title="Pool State">
                    <div className="text-gray-300 space-y-2">
                        <p><strong>Mint X:</strong> {poolState.mintX.toBase58()}</p>
                        <p><strong>Mint Y:</strong> {poolState.mintY.toBase58()}</p>
                        <p><strong>LP Mint:</strong> {poolState.lpMint.toBase58()}</p>
                        <p><strong>Vault X Balance:</strong> {(Number(poolState.vaultXAmount) / (10 ** 6)).toLocaleString()}</p>
                        <p><strong>Vault Y Balance:</strong> {(Number(poolState.vaultYAmount) / (10 ** 6)).toLocaleString()}</p>
                        <p><strong>Fee:</strong> {poolState.fee / 100}%</p>
                    </div>
                </Card>
            )}

            {txSignature && (
                <div className="text-center text-gray-400 mt-4">
                    <p>Last Transaction:</p>
                    <a
                        href={`https://explorer.solana.com/tx/${txSignature}?cluster=devnet`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-purple-400 hover:underline break-all"
                    >
                        {txSignature}
                    </a>
                </div>
            )}
        </div>
    );
}
