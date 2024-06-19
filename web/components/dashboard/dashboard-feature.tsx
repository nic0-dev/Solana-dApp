'use client';

import { AppHero } from '../ui/ui-layout';

export default function DashboardFeature() {
  return (
    <div>
      <AppHero title="Narwhal dApp" subtitle="dApp using Solana and Anchor" />
      <div className="max-w-xl mx-auto py-6 sm:px-6 lg:px-8 text-center">
        <div className="space-y-2">
        </div>
      </div>
    </div>
  );
}
