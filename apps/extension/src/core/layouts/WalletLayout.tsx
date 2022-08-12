// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

import React, { useMemo } from 'react';
import { Box, Grid, useColorMode } from '@chakra-ui/react';
import WalletFooter from 'core/components/WalletFooter';
import WalletHeader from 'core/components/WalletHeader';
import { secondaryBgColor } from 'core/colors';

interface WalletLayoutProps {
  children: React.ReactNode;
  hasWalletFooter?: boolean;
  hasWalletHeader?: boolean;
  showBackButton?: boolean;
}

export default function WalletLayout({
  children,
  hasWalletFooter = true,
  hasWalletHeader = true,
  showBackButton,
}: WalletLayoutProps) {
  const { colorMode } = useColorMode();

  const templateRows = useMemo(() => {
    if (hasWalletFooter && hasWalletHeader) {
      return '64px 1fr 60px';
    }
    if (hasWalletFooter) {
      return '1fr 40px';
    }
    if (hasWalletHeader) {
      return '64px 1fr';
    }
    return '1fr';
  }, [hasWalletHeader, hasWalletFooter]);

  return (
    <Grid
      height="100%"
      width="100%"
      maxW="100%"
      templateRows={templateRows}
      bgColor={secondaryBgColor[colorMode]}
    >
      {hasWalletHeader ? (
        <WalletHeader showBackButton={showBackButton} />
      ) : undefined}
      <Box maxH="100%" overflowY="auto" pb={4}>
        {children}
      </Box>
      {hasWalletFooter ? (
        <WalletFooter />
      ) : undefined}
    </Grid>
  );
}
