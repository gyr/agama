# frozen_string_literal: true

# Copyright (c) [2023] SUSE LLC
#
# All Rights Reserved.
#
# This program is free software; you can redistribute it and/or modify it
# under the terms of version 2 of the GNU General Public License as published
# by the Free Software Foundation.
#
# This program is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
# more details.
#
# You should have received a copy of the GNU General Public License along
# with this program; if not, contact SUSE LLC.
#
# To contact SUSE LLC about this file by physical or electronic mail, you may
# find current contact information at www.suse.com.

require "agama/dbus/storage/dasds_tree"
require "agama/dbus/storage/jobs_tree"
require "agama/storage/dasd/manager"

module Agama
  module DBus
    module Storage
      module Interfaces
        # Mixin to define the D-Bus interface to manage DASD devices
        #
        # @note This mixin is expected to be included by {Agama::DBus::Storage::Manager}.
        module DasdManager
          DASD_MANAGER_INTERFACE = "org.opensuse.Agama.Storage1.DASD.Manager"
          private_constant :DASD_MANAGER_INTERFACE

          def self.included(base)
            base.class_eval do
              dbus_interface DASD_MANAGER_INTERFACE do
                # Finds DASDs in the system and populates the D-Bus objects tree according
                dbus_method(:Probe) do
                  busy_while { dasd_backend.probe }
                end

                # Enables the given list of DASDs.
                # See documentation at Agama::Storage::DASD::Manager to understand how the
                # use_diag flag is affected by this operation.
                dbus_method(:Enable, "in devices:ao, out result:u") do |devs|
                  busy_while { dasd_enable(devs) }
                end

                # Disables the given list of DASDs
                dbus_method(:Disable, "in devices:ao, out result:u") do |devs|
                  busy_while { dasd_disable(devs) }
                end

                # Sets the use_diag attribute for the given DASDs to the given value.
                # See documentation at Agama::Storage::DASD::Manager to understand what that
                # really means (since this follows the same strange convention than YaST).
                dbus_method(:SetDiag, "in devices:ao, in diag:b, out result:u") do |devs, diag|
                  busy_while { dasd_set_diag(devs, diag) }
                end

                # Creates a format job for the DASDs in the given list
                dbus_method(:Format, "in devices:ao, out result:u, out job:o") do |devs|
                  busy_while { dasd_format(devs) }
                end
              end
            end
          end

          # Enables the DASDs devices indentified by the given D-Bus paths
          #
          # See {#dasds_dbus_method} for information about the possible return codes.
          #
          # Succeeds (returns 0) if all the devices where successfully enabled.
          #
          # @param paths [Array<String>]
          # @return [Integer]
          def dasd_enable(paths)
            dasds_dbus_method(paths) { |dasds| dasd_backend.enable(dasds) }
          end

          # Disables the DASDs devices indentified by the given D-Bus paths
          #
          # See {#dasds_dbus_method} for information about the possible return codes.
          #
          # Succeeds (returns 0) if all the devices where successfully disabled.
          #
          # @param paths [Array<String>]
          # @return [Integer]
          def dasd_disable(paths)
            dasds_dbus_method(paths) { |dasds| dasd_backend.disable(dasds) }
          end

          # Sets use_diag to the given value for the DASDs devices indentified by the given
          # D-Bus paths
          #
          # See {#dasds_dbus_method} for information about the possible return codes.
          #
          # Succeeds (returns 0) if the desired value of use_diag is properly set for all the
          # given devices.
          #
          # @param paths [Array<String>]
          # @param diag [Boolean] value of the use_diag flag
          # @return [Integer]
          def dasd_set_diag(paths, diag)
            dasds_dbus_method(paths) { |dasds| dasd_backend.set_diag(dasds, diag) }
          end

          # Format the DASDs devices indentified by the given D-Bus paths
          #
          # See {#dasds_dbus_method} for information about the possible return codes.
          #
          # Succeeds (returns 0) if all the devices where successfully formatted.
          #
          # @param paths [Array<String>]
          # @return [Array(Integer,::DBus::ObjectPath)]
          def dasd_format(paths)
            dasds = find_dasds(paths)
            return [1, "/"] if dasds.nil?

            # Theoreticaly, there is room for a race condition here if the callbacks 'progress' or
            # 'finish' are called before the job is created below. But in practice it will not
            # happen because dasd_backend#format sleeps before calling any of the callbacks and, of
            # course, it only calls them if the formatting process effectively started.
            #
            # We can change the approach in the future and always create the job beforehand if we
            # feel the risk is not acceptable. That would make the Format operation a bit less
            # consistent with other methods in this interface. If the format process cannot be
            # started it would still return 0 as result and would create a job in the tree with kind
            # of meaningless progress information representing the failed execution.
            job = nil
            progress = proc { |statuses| job.update_format(statuses) }
            finish = proc { |result| job.finish_format(result) }
            initial_statuses = dasd_backend.format(dasds, on_progress: progress, on_finish: finish)
            return [2, "/"] unless initial_statuses

            job = jobs_tree.add_dasds_format(initial_statuses, dasds_tree)
            [0, job.path]
          end

          # Finds the DASDs matching the given D-Bus paths and calls the given block on them
          #
          # It only tries to execute the given block if all the paths are found. Otherwise it
          # returns 1 without trying to process any of the DASDs.
          #
          # If all the paths are on the D-Bus tree, it runs the block and returns 0 if it succeeds
          # or 2 otherwise.
          #
          # @return [Integer]
          def dasds_dbus_method(paths)
            dasds = find_dasds(paths)
            return 1 if dasds.nil?

            return 0 if yield(dasds)

            2
          end

          # Finds the DASDs matching the given D-Bus paths
          #
          # Returns nil (and logs an error) unless all the paths are known.
          #
          # @return [Array<Y2S390::Dasd>, nil]
          def find_dasds(paths)
            dbus_dasds = dasds_tree.find_paths(paths)
            if dbus_dasds.size != paths.size
              missing = paths - dbus_dasds.map(&:path)
              logger.error "Unknown path(s) #{missing} at requested list #{paths}"
              return nil
            end

            dbus_dasds.map(&:dasd)
          end

          # Registers the callbacks necessary to ensure accuracy of the tree of DASD objects
          def register_dasd_callbacks
            dasd_backend.on_probe do |dasds|
              dasds_tree.populate(dasds)
              deprecate_system
            end

            dasd_backend.on_refresh do |dasds|
              dasds_tree.update(dasds)
              deprecate_system
            end
          end

          # Tree with the devices representing the DASDs in the system
          def dasds_tree
            @dasds_tree ||= Storage::DasdsTree.new(@service, logger: logger)
          end

          # Tree with the storage jobs
          def jobs_tree
            @jobs_tree ||= Storage::JobsTree.new(@service, logger: logger)
          end

          # DASD manager
          #
          # @return [Storage::DASD::Manager]
          def dasd_backend
            @dasd_backend ||= Agama::Storage::DASD::Manager.new(logger: logger)
          end
        end
      end
    end
  end
end
