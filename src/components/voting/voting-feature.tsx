import { WalletButton } from '../solana/solana-provider'
import { VotingCreate, VotingProgram, VotingProgramExplorerLink } from './voting-ui'
import { AppHero } from '../app-hero'
import { useWalletUi } from '@wallet-ui/react'

export default function VotingFeature() {
  const { account } = useWalletUi()

  if (!account) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="hero py-[64px]">
          <div className="hero-content text-center">
            <WalletButton />
          </div>
        </div>
      </div>
    )
  }

  return (
    <div>
      <AppHero title="Voting" subtitle={'Run the program by clicking the "Run program" button.'}>
        <p className="mb-6">
          <VotingProgramExplorerLink />
        </p>
        <VotingCreate />
      </AppHero>
      <VotingProgram />
    </div>
  )
}
